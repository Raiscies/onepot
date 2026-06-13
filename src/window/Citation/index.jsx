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
import { MdPictureAsPdf } from 'react-icons/md';
import { MdSearch } from 'react-icons/md';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { Button } from '@nextui-org/react';
import toast, { Toaster } from 'react-hot-toast';
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
    const [searchGen, setSearchGen] = useState(0);
    // Listen for backend events: citation_init, citation_update
    useEffect(() => {
        const unlisteners = [];

        // Pull state on mount in case event was emitted before load
        invoke('get_citation_state').then((raw) => {
            const data = JSON.parse(raw);
            if (data.captured_text) {
                setCapturedText(data.captured_text);
                setResults(data.papers || []);
                setSearchGen((g) => g + 1);
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
            setSearchGen((g) => g + 1);
            if (!hideWindow) {
                appWindow.show();
                appWindow.setFocus();
            }
        }).then((f) => unlisteners.push(f));

        // citation_update: deep-merge into existing card, missing fields preserved
        listen('citation_update', (event) => {
            const { index, phase, data } = JSON.parse(event.payload);
            data._searchPhase = phase;
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

    // Compute whether auto-download should trigger
    const [autoDlCountRaw] = useConfig('download_auto_count', '0');
    const autoDlCountGlobal = parseInt(autoDlCountRaw, 10) || 0;
    const parsedCount = results.filter((r) => r.paper?.status !== 'error' && r.paper?.status !== 'searching').length;
    const allDone = results.length > 0 && !results.some((r) => r.paper?.status === 'searching');
    const shouldAutoDownload = allDone && autoDlCountGlobal > 0 && parsedCount <= autoDlCountGlobal;

    return (
        <div className='flex flex-col h-screen bg-background'>
            <Toaster />
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
                {[...results]
                    .sort((a, b) => (a.paper?.status === 'error' ? 1 : 0) - (b.paper?.status === 'error' ? 1 : 0))
                    .map((item) => (
                    <PaperCardItem key={`${searchGen}-${item.index}`} item={item} shouldAutoDownload={shouldAutoDownload} />
                ))}
            </div>
        </div>
    );
}

function PaperCardItem({ item, shouldAutoDownload }) {
    const p = item.paper;
    const isError = p.status === 'error';
    const isSearching = p.status === 'searching';
    const searchPhase = item._searchPhase || 'parsing';
    const showSearchBar = (isSearching || searchPhase === 'parsed') && !isError;
    const [collapsed, setCollapsed] = useState(isError);
    const [contentRef, bounds] = useMeasure({ scroll: true });

    const [copiedTitle, setCopiedTitle] = useState(false);
    const [copiedAuthor, setCopiedAuthor] = useState(null);
    const [searchEngineRaw] = useConfig('citation_search_engine', '');
    const searchEngine = searchEngineRaw || 'https://scholar.google.com/scholar?q={query}';
    const [downloadState, setDownloadState] = useState('idle');
    const [downloadPath, setDownloadPath] = useState('');
    const [downloadProgress, setDownloadProgress] = useState({ downloaded: 0, total: 0 });
    const [autoOpenPdfCard] = useConfig('download_auto_open', false);
    const [autoOpenUrl] = useConfig('download_auto_open_doi', false);
    const autoDownloadedRef = React.useRef(false);

    /// Perform the actual download. Shared by manual click and auto-download.
    const doDownload = async () => {
        setDownloadState('downloading');
        setDownloadProgress({ downloaded: 0, total: 0 });
        try {
            const raw = await invoke('download_citation_pdf', { doi: p.doi, paper: p });
            const outcome = JSON.parse(raw);
            if (outcome.status === 'success') {
                setDownloadState('success');
                setDownloadPath(outcome.path);
                if (autoOpenPdfCard) open(outcome.path);
            } else if (outcome.status === 'no_handler') {
                setDownloadState('no_handler');
                toast.error(`No download handler for: ${outcome.host}`);
                if (autoOpenUrl && p.url) open(p.url);
            } else {
                setDownloadState('failed');
                toast.error(outcome.reason || 'Download failed');
                if (autoOpenUrl && p.url) open(p.url);
            }
        } catch (e) {
            setDownloadState('failed');
            toast.error(`Download error: ${e}`);
            if (autoOpenUrl && p.url) open(p.url);
        }
    };

    // Auto-download: parent signals when conditions are met, card executes
    useEffect(() => {
        if (!shouldAutoDownload || !p.doi || isError) return;
        if (autoDownloadedRef.current) return;
        autoDownloadedRef.current = true;
        doDownload();
    }, [shouldAutoDownload, p.doi, isError]);

    // Listen for download progress and finished events
    useEffect(() => {
        if (!p.doi) return;
        const unlisteners = [];

        listen('download_progress', (event) => {
            const data = JSON.parse(event.payload);
            if (data.doi === p.doi) {
                setDownloadState('downloading');
                setDownloadProgress({ downloaded: data.downloaded, total: data.total });
            }
        }).then((f) => unlisteners.push(f));

        listen('download_finished', (event) => {
            const data = JSON.parse(event.payload);
            if (data.doi === p.doi) {
                if (data.status === 'success') {
                    setDownloadState('success');
                    setDownloadPath(data.path);
                } else {
                    setDownloadState('failed');
                }
            }
        }).then((f) => unlisteners.push(f));

        // Check if PDF already exists in cache on mount
        if (!isSearching && !isError) {
            invoke('check_pdf_exists', { doi: p.doi }).then((path) => {
                if (path) {
                    setDownloadState('success');
                    setDownloadPath(path);
                }
            });
        }
        return () => { unlisteners.forEach((f) => f()); };
    }, [p.doi, isSearching, isError]);

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
        ? 'Unknown'
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
                    {p.doi && downloadState !== 'no_handler' && downloadState !== 'success' && (
                        <Button
                            isIconOnly
                            size='sm'
                            variant='light'
                            className='min-w-0 w-6 h-6'
                            isDisabled={downloadState === 'downloading'}
                            onPress={doDownload}
                        >
                            <MdFileDownload className='text-small' />
                        </Button>
                    )}
                    {downloadState === 'success' && (
                        <Button
                            isIconOnly
                            size='sm'
                            variant='light'
                            className='min-w-0 w-6 h-6'
                            onPress={async () => {
                                // Verify file still exists before opening
                                const path = await invoke('check_pdf_exists', { doi: p.doi });
                                if (path) {
                                    open(path);
                                } else {
                                    setDownloadState('idle');
                                    setDownloadPath('');
                                }
                            }}
                        >
                            <MdPictureAsPdf className='text-small' />
                        </Button>
                    )}
                    {downloadState === 'no_handler' && (
                        <Button
                            isIconOnly
                            size='sm'
                            variant='light'
                            className='min-w-0 w-6 h-6'
                            isDisabled
                        >
                            <MdFileDownload className='text-small text-default-300' />
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
                    {p.url && (
                        <Button
                            isIconOnly
                            size='sm'
                            variant='light'
                            className='min-w-0 w-6 h-6'
                            onPress={() => open(p.doi ? `https://doi.org/${p.doi}` : p.url)}
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

                    {/* unified progress bar slot */}
                    {downloadState === 'downloading' ? (
                        <div className='mt-2 h-1 bg-default-100 rounded-full overflow-hidden'>
                            {downloadProgress.total > 0 ? (
                                <div
                                    className='h-full bg-green-400 rounded-full transition-all duration-300'
                                    style={{ width: `${Math.round((downloadProgress.downloaded / downloadProgress.total) * 100)}%` }}
                                />
                            ) : (
                                <div className='h-full w-1/3 bg-green-400 rounded-full animate-indeterminate' />
                            )}
                        </div>
                    ) : showSearchBar ? (
                        <div className='mt-2 h-1 bg-default-100 rounded-full overflow-hidden'>
                            <div className={`h-full w-1/3 rounded-full animate-indeterminate ${searchPhase === 'parsing' ? 'bg-default-500' : 'bg-primary'}`} />
                        </div>
                    ) : null}

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
