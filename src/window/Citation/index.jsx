import { appWindow } from '@tauri-apps/api/window';
import { writeText } from '@tauri-apps/api/clipboard';
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
import { Button } from '@nextui-org/react';
import React, { useState, useEffect } from 'react';

import { useConfig } from '../../hooks';
import { osType } from '../../utils/env';

const originalCitation = '[1] Smith, J. et al. (2023). A great paper. JMLR, 24, 1-20.\\n[2] Doe, A. (2022). Another study. Nature, 600, 100-110.';

let blurTimeout = null;

const listenBlur = () => {
    return listen('tauri://blur', () => {
        if (appWindow.label === 'citation') {
            if (blurTimeout) clearTimeout(blurTimeout);
            blurTimeout = setTimeout(async () => {
                await appWindow.close();
            }, 100);
        }
    });
};

let unlisten = listenBlur();
const unlistenBlur = () => {
    unlisten.then((f) => f());
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
    const [results, setResults] = useState([]);

    useEffect(() => {
        setResults([
            {
                paper: {
                    title: 'A Sample Research Paper on Machine Learning',
                    authors: ['Smith, J.', 'Doe, A.'],
                    year: 2023,
                    doi: '10.1234/sample.2023',
                    journal: 'Journal of AI Research',
                    volume: '42',
                    pages: '1-15',
                    publisher: 'ACM',
                    url: 'https://doi.org/10.1234/sample.2023',
                    status: 'ready',
                    tldr: 'This paper presents a novel approach to machine learning that achieves state-of-the-art results.',
                    abstract: null,
                    citation_count: 42,
                    ccf_rank: 'A',
                },
                index: 0,
                citation_index: '1',
                raw_citation: '[1] Smith, J. et al. (2023). A Sample Research Paper...',
                status: 'ready',
            },
            {
                paper: {
                    title: 'Another Interesting Study',
                    authors: ['Johnson, R.'],
                    year: 2022,
                    doi: '10.5678/another.2022',
                    journal: null,
                    volume: null,
                    pages: null,
                    publisher: 'Springer',
                    url: 'https://doi.org/10.5678/another.2022',
                    status: 'ready',
                    tldr: null,
                    abstract: 'We investigate the properties of a novel material under extreme conditions, revealing unexpected phase transitions and quantum effects that challenge existing theoretical frameworks.',
                    citation_count: 156,
                    ccf_rank: null,
                },
                index: 1,
                citation_index: '2',
                raw_citation: '[2] Johnson, R. (2022). Another Interesting Study...',
                status: 'ready',
            },
        ]);
    }, []);

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
                        if (pined) {
                            if (closeOnBlur) {
                                unlisten = listenBlur();
                            }
                            appWindow.setAlwaysOnTop(false);
                        } else {
                            unlistenBlur();
                            appWindow.setAlwaysOnTop(true);
                        }
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
                <div className='text-tiny text-default-400 bg-default-100 rounded-lg p-2 max-h-[80px] overflow-y-auto whitespace-pre-wrap'>
                    {originalCitation}
                </div>
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

    const headerTitle = isSearching ? 'Searching...' : isError ? 'Parse Error' : p.title || 'Untitled';

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
                        <Button isIconOnly size='sm' variant='light' className='min-w-0 w-6 h-6'>
                            <MdFileDownload className='text-small' />
                        </Button>
                    )}
                    <Button isIconOnly size='sm' variant='light' className='min-w-0 w-6 h-6'>
                        <MdSearch className='text-small' />
                    </Button>
                    {p.doi && (
                        <Button isIconOnly size='sm' variant='light' className='min-w-0 w-6 h-6'>
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
                            className='text-tiny text-default-500 mt-1.5 italic cursor-pointer hover:text-primary'
                            onClick={() => writeText(p.tldr)}
                        >
                            {p.tldr}
                        </div>
                    )}

                    {/* abstract */}
                    {p.abstract && (
                        <div
                            className='text-tiny text-default-500 mt-1.5 leading-relaxed cursor-pointer hover:text-primary'
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
