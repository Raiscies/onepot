import { appWindow } from '@tauri-apps/api/window';
import { writeText } from '@tauri-apps/api/clipboard';
import { open } from '@tauri-apps/api/shell';
import { useSpring, animated } from '@react-spring/web';
import { AiFillCloseCircle } from 'react-icons/ai';
import { BiCollapseVertical, BiExpandVertical } from 'react-icons/bi';
import { BsPinFill } from 'react-icons/bs';
import useMeasure from 'react-use-measure';
import { Chip } from '@nextui-org/react';
import { MdFileDownload } from 'react-icons/md';
import { MdOpenInNew } from 'react-icons/md';
import { MdSearch } from 'react-icons/md';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { Button } from '@nextui-org/react';
import React, { useState, useEffect } from 'react';

import { useConfig } from '../../hooks';
import { osType } from '../../utils/env';

/// Deep-merge `src` into `dst`. Missing keys and null values in src are ignored.
function deepMerge(dst, src) {
    if (src === null || src === undefined) return dst;
    if (typeof src !== 'object' || Array.isArray(src)) return src;
    const result = { ...dst };
    for (const key of Object.keys(src)) {
        if (src[key] === null || src[key] === undefined) continue;
        if (typeof src[key] === 'object' && !Array.isArray(src[key]) && dst[key] != null) {
            result[key] = deepMerge(dst[key], src[key]);
        } else {
            result[key] = src[key];
        }
    }
    return result;
}

let blurTimeout = null;

const listenBlur = (closeOnBlur) => {
    return listen('tauri://blur', () => {
        if (!closeOnBlur) return;
        if (appWindow.label === 'citation') {
            if (blurTimeout) clearTimeout(blurTimeout);
            blurTimeout = setTimeout(async () => {
                await appWindow.close();
            }, 100);
        }
    });
};

let unlisten = null;
const unlistenBlur = () => {
    if (unlisten) unlisten.then((f) => f());
    unlisten = null;
};

void listen('tauri://focus', () => {
    if (blurTimeout) clearTimeout(blurTimeout);
});
void listen('tauri://move', () => {
    if (blurTimeout) clearTimeout(blurTimeout);
});

export default function Citation() {
    const [pined, setPined] = useState(false);
    const [closeOnBlur] = useConfig('citation_close_on_blur', true);
    const [alwaysOnTop] = useConfig('citation_always_on_top', false);
    const [capturedText, setCapturedText] = useState('');
    const [results, setResults] = useState([]);
    const [hideCitationText] = useConfig('citation_hide_citation_text', false);
    const [hideWindow] = useConfig('citation_hide_window', false);

    // Listen for backend events: citation_init, citation_update
    useEffect(() => {
        const unlisteners = [];

        // Pull state on mount in case event was emitted before load
        invoke('get_citation_state').then((raw) => {
            const data = JSON.parse(raw);
            if (data.captured_text) {
                setCapturedText(data.captured_text);
                setResults(data.papers || []);
                if (!hideWindow) {
                    appWindow.show();
                    appWindow.setFocus();
                }
            }
        });

        listen('citation_init', (event) => {
            const data = JSON.parse(event.payload);
            setCapturedText(data.captured_text || '');
            setResults(data.papers || []);
            if (!hideWindow) {
                appWindow.show();
                appWindow.setFocus();
            }
        }).then((f) => unlisteners.push(f));

        // citation_update: deep-merge into existing card, missing fields preserved
        listen('citation_update', (event) => {
            const { index, data } = JSON.parse(event.payload);
            setResults((prev) => {
                const next = [...prev];
                if (index >= 0 && index < next.length) {
                    next[index] = deepMerge(next[index], data);
                }
                return next;
            });
        }).then((f) => unlisteners.push(f));

        return () => {
            unlisteners.forEach((f) => f());
        };
    }, []);

    // Blur listener respects closeOnBlur setting
    useEffect(() => {
        if (closeOnBlur && !pined) {
            unlisten = listenBlur(closeOnBlur);
        } else {
            unlistenBlur();
        }
        return () => unlistenBlur();
    }, [closeOnBlur, pined]);

    return (
        <div className='flex flex-col h-screen bg-background'>
            <div data-tauri-drag-region='true' className='fixed top-[5px] left-[5px] right-[5px] h-[30px]' />
            <div className={`h-[35px] w-full flex ${osType === 'Darwin' ? 'justify-end' : 'justify-between'}`}>
                <Button
                    isIconOnly
                    size='sm'
                    variant='flat'
                    disableAnimation
                    className='my-auto bg-transparent'
                    onPress={() => {
                        appWindow.setAlwaysOnTop(!pined);
                        setPined(!pined);
                    }}
                >
                    <BsPinFill className={`text-[20px] ${pined ? 'text-primary' : 'text-default-400'}`} />
                </Button>
                <Button
                    isIconOnly
                    size='sm'
                    variant='flat'
                    disableAnimation
                    className='my-auto bg-transparent'
                    onPress={() => appWindow.close()}
                >
                    <AiFillCloseCircle className='text-[20px] text-default-400' />
                </Button>
            </div>
            <div className='px-2 pb-1'>
                {!hideCitationText && capturedText && (
                    <div className='text-tiny text-default-400 bg-default-100 rounded-lg p-2 max-h-[80px] overflow-y-auto whitespace-pre-wrap'>
                        {capturedText}
                    </div>
                )}
            </div>
            <div className='flex-1 overflow-y-auto px-2 pb-2'>
                {results.map((item) => (
                    <PaperCardItem key={item.index} item={item} />
                ))}
            </div>
        </div>
    );
}

function PaperCardItem({ item }) {
    const p = item.paper;
    const isError = p.status === 'error';
    const isSearching = p.status === 'searching';
    const [collapsed, setCollapsed] = useState(false);
    const [contentRef, bounds] = useMeasure({ scroll: true });

    const [copiedTitle, setCopiedTitle] = useState(false);
    const [copiedAuthor, setCopiedAuthor] = useState(null);
    const [searchEngineRaw] = useConfig('citation_search_engine', '');
    const searchEngine = searchEngineRaw || 'https://scholar.google.com/scholar?q={query}';
    const [downloadMsg, setDownloadMsg] = useState('');

    const springs = useSpring({
        from: { height: 0 },
        to: { height: collapsed ? 0 : bounds.height },
    });

    function copyTitle() {
        const text = p.title || '';
        if (!text) return;
        writeText(text);
        setCopiedTitle(true);
        setTimeout(() => setCopiedTitle(false), 800);
    }

    function copyAuthor(author, idx) {
        writeText(author);
        setCopiedAuthor(idx);
        setTimeout(() => setCopiedAuthor(null), 800);
    }

    const headerTitle = isSearching
        ? (item.raw_citation || 'Searching...')
        : isError
        ? 'Parse Error'
        : p.title || 'Untitled';

    return (
        <div className='mb-2 rounded-lg border border-divider bg-content1 overflow-hidden'>
            {/* header */}
            <div className='flex items-center gap-1 px-2 py-1.5'>
                <div
                    className='flex-1 flex items-center gap-1 min-w-0 cursor-pointer'
                    onClick={() => setCollapsed(!collapsed)}
                >
                    {item.citation_index && !isSearching && !isError && (
                        <span className='text-tiny font-medium text-blue-400 shrink-0'>
                            [{item.citation_index}]
                        </span>
                    )}
                    <span
                        className={`text-tiny font-medium hover:text-primary ${
                            copiedTitle ? 'text-success' : ''
                        }`}
                        onClick={(e) => {
                            e.stopPropagation();
                            copyTitle();
                        }}
                    >
                        {copiedTitle ? 'Copied!' : headerTitle}
                    </span>
                </div>
                <div className='flex gap-0.5'>
                    {p.doi && (
                        <Button
                            isIconOnly
                            size='sm'
                            variant='light'
                            className='min-w-0 w-6 h-6'
                            onPress={async () => {
                                setDownloadMsg('Downloading...');
                                try {
                                    const path = await invoke('download_citation_pdf', { doi: p.doi, paper: p });
                                    setDownloadMsg(`Saved: ${path}`);
                                } catch (e) {
                                    setDownloadMsg(`Failed: ${e}`);
                                }
                                setTimeout(() => setDownloadMsg(''), 3000);
                            }}
                        >
                            <MdFileDownload className='text-small' />
                        </Button>
                    )}
                    <Button
                        isIconOnly
                        size='sm'
                        variant='light'
                        className='min-w-0 w-6 h-6'
                        onPress={() => {
                            const rawQuery = encodeURIComponent(item.raw_citation || p.title || '');
                            const titleQuery = encodeURIComponent(p.title || '');
                            const url = searchEngine
                                .replace('{query}', rawQuery)
                                .replace('{title}', titleQuery);
                            open(url);
                        }}
                    >
                        <MdSearch className='text-small' />
                    </Button>
                    {p.doi && (
                        <Button
                            isIconOnly
                            size='sm'
                            variant='light'
                            className='min-w-0 w-6 h-6'
                            onPress={() => open(`https://doi.org/${p.doi}`)}
                        >
                            <MdOpenInNew className='text-small' />
                        </Button>
                    )}
                    <Button
                        isIconOnly
                        size='sm'
                        variant='light'
                        className='min-w-0 w-6 h-6'
                        onPress={() => setCollapsed(!collapsed)}
                    >
                        {collapsed ? (
                            <BiExpandVertical className='text-tiny' />
                        ) : (
                            <BiCollapseVertical className='text-tiny' />
                        )}
                    </Button>
                </div>
            </div>
            {/* animated body */}
            <animated.div style={{ overflow: 'hidden', ...springs }}>
                <div ref={contentRef} className='px-2 pb-2'>
                    {/* authors as clickable chips */}
                    {p.authors && p.authors.length > 0 && (
                        <div className='flex items-center justify-between gap-2 mb-1.5'>
                            <div className='flex flex-wrap gap-1'>
                                {p.authors.map((author, i) => (
                                    <Chip
                                        key={i}
                                        size='sm'
                                        variant='flat'
                                        color={copiedAuthor === i ? 'success' : 'primary'}
                                        className='text-tiny cursor-pointer transition-colors'
                                        onClick={() => copyAuthor(author, i)}
                                    >
                                        {copiedAuthor === i ? 'Copied!' : author}
                                    </Chip>
                                ))}
                            </div>
                            {(p.year || p.citation_count != null) && (
                                <span className='text-tiny text-default-400 whitespace-nowrap shrink-0'>
                                    {p.year}{p.citation_count != null ? ` | Cited: ${p.citation_count}` : ''}
                                </span>
                            )}
                        </div>
                    )}

                    {/* journal / venue + CCF rank */}
                    {(p.journal || p.volume || p.pages || p.ccf_rank) && (
                        <div className='flex items-center justify-between gap-2 mb-1'>
                            <div
                                className='text-tiny text-default-400 cursor-pointer hover:text-primary'
                                onClick={() => {
                                    const parts = [p.journal, p.volume && `vol. ${p.volume}`, p.pages && `pp. ${p.pages}`].filter(Boolean);
                                    if (parts.length) writeText(parts.join(', '));
                                }}
                            >
                                {[p.journal, p.volume && `vol. ${p.volume}`, p.pages && `pp. ${p.pages}`]
                                    .filter(Boolean)
                                    .join(', ')}
                            </div>
                            {p.ccf_rank && (
                                <Chip size='sm' variant='flat' color='warning' className='text-tiny h-5 shrink-0'>
                                    CCF-{p.ccf_rank}
                                </Chip>
                            )}
                        </div>
                    )}

                    {/* DOI clickable copy */}
                    {p.doi && (
                        <div
                            className='text-tiny text-default-400 cursor-pointer hover:text-primary'
                            onClick={() => writeText(p.doi)}
                        >
                            DOI: {p.doi}
                        </div>
                    )}

                    {/* TLDR (only if no abstract) */}
                    {!p.abstract && p.tldr && (
                        <div
                            className='text-tiny text-default-500 mt-1.5 italic cursor-pointer hover:text-primary max-h-20 overflow-y-auto'
                            onClick={() => writeText(p.tldr)}
                        >
                            {p.tldr}
                        </div>
                    )}

                    {/* abstract */}
                    {p.abstract && (
                        <div
                            className='text-tiny text-default-500 mt-1.5 leading-relaxed cursor-pointer hover:text-primary max-h-24 overflow-y-auto'
                            onClick={() => writeText(p.abstract)}
                        >
                            {p.abstract}
                        </div>
                    )}

                    {/* search progress bar */}
                    {isSearching && (
                        <div className='mt-2 h-1 bg-default-100 rounded-full overflow-hidden'>
                            <div className='h-full w-1/3 bg-primary rounded-full animate-pulse' />
                        </div>
                    )}

                    {/* raw citation on error */}
                    {isError && item.raw_citation && (
                        <div className='text-tiny text-default-400 mt-1 italic'>
                            {item.raw_citation}
                        </div>
                    )}
                </div>
            </animated.div>
        </div>
    );
}
