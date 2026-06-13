import { DropdownTrigger } from '@nextui-org/react';
import { DropdownMenu } from '@nextui-org/react';
import { DropdownItem } from '@nextui-org/react';
import { Input } from '@nextui-org/react';
import { useTranslation } from 'react-i18next';
import { CardBody } from '@nextui-org/react';
import { Dropdown } from '@nextui-org/react';
import { Switch } from '@nextui-org/react';
import { Button } from '@nextui-org/react';
import { Card } from '@nextui-org/react';
import React from 'react';

import { useConfig } from '../../../../hooks/useConfig';
import { invoke } from '@tauri-apps/api';

export default function Citation() {
    const [windowPosition, setWindowPosition] = useConfig('citation_window_position', 'mouse');
    const [rememberWindowSize, setRememberWindowSize] = useConfig('citation_remember_window_size', false);
    const [closeOnBlur, setCloseOnBlur] = useConfig('citation_close_on_blur', true);
    const [alwaysOnTop, setAlwaysOnTop] = useConfig('citation_always_on_top', false);
    const [hideCitationText, setHideCitationText] = useConfig('citation_hide_citation_text', false);
    const [hideWindow, setHideWindow] = useConfig('citation_hide_window', false);
    const [rubyPath, setRubyPath] = useConfig('citation_ruby_path', '');
    const [searchEngine, setSearchEngine] = useConfig('citation_search_engine', '');
    const [rubyStatus, setRubyStatus] = React.useState('');
    const { t } = useTranslation();

    const searchEnginePresets = [
        { key: 'scholar', label: 'Google Scholar', url: 'https://scholar.google.com/scholar?q={query}' },
        { key: 'google', label: 'Google', url: 'https://www.google.com/search?q={query}' },
        { key: 'dblp', label: 'DBLP', url: 'https://dblp.org/search?q={title}' },
        { key: 'bing', label: 'Bing', url: 'https://www.bing.com/search?q={query}' },
        { key: 'arxiv', label: 'arXiv', url: 'https://arxiv.org/search/?query={title}&searchtype=all' },
    ];

    return (
        <Card>
            <CardBody>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.window_position')}</h3>
                    {windowPosition !== null && (
                        <Dropdown>
                            <DropdownTrigger>
                                <Button variant='bordered'>{t(`config.citation.${windowPosition}`)}</Button>
                            </DropdownTrigger>
                            <DropdownMenu
                                aria-label='window position'
                                className='max-h-[50vh] overflow-y-auto'
                                onAction={(key) => {
                                    setWindowPosition(key);
                                }}
                            >
                                <DropdownItem key='mouse'>{t('config.citation.mouse')}</DropdownItem>
                                <DropdownItem key='pre_state'>{t('config.citation.pre_state')}</DropdownItem>
                            </DropdownMenu>
                        </Dropdown>
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.remember_window_size')}</h3>
                    {rememberWindowSize !== null && (
                        <Switch
                            isSelected={rememberWindowSize}
                            onValueChange={(v) => {
                                setRememberWindowSize(v);
                            }}
                        />
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.close_on_blur')}</h3>
                    {closeOnBlur !== null && (
                        <Switch
                            isSelected={closeOnBlur}
                            onValueChange={(v) => {
                                setCloseOnBlur(v);
                            }}
                        />
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.always_on_top')}</h3>
                    {alwaysOnTop !== null && (
                        <Switch
                            isSelected={alwaysOnTop}
                            onValueChange={(v) => {
                                setAlwaysOnTop(v);
                            }}
                        />
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.hide_citation_text')}</h3>
                    {hideCitationText !== null && (
                        <Switch
                            isSelected={hideCitationText}
                            onValueChange={(v) => {
                                setHideCitationText(v);
                            }}
                        />
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.hide_window')}</h3>
                    {hideWindow !== null && (
                        <Switch
                            isSelected={hideWindow}
                            onValueChange={(v) => {
                                setHideWindow(v);
                            }}
                        />
                    )}
                </div>
                <div className='config-item mt-4 pt-3 border-t border-divider'>
                    <div className='flex items-center gap-2 w-full'>
                        <Input
                            variant='bordered'
                            label={t('config.citation.ruby_path')}
                            value={rubyPath}
                            onValueChange={(v) => {
                                setRubyPath(v);
                                invoke('reinit_ruby');
                            }}
                            className='flex-1'
                            size='sm'
                        />
                        <Button
                            size='sm'
                            variant='flat'
                            onPress={async () => {
                                const result = await invoke('test_ruby_path', { path: rubyPath });
                                setRubyStatus(result);
                            }}
                        >
                            {t('config.citation.test_ruby')}
                        </Button>
                    </div>
                    {rubyStatus && (
                        <div className='text-tiny text-default-500 mt-1 whitespace-pre-wrap'>
                            {rubyStatus}
                        </div>
                    )}
                </div>
                <div className='config-item mt-4 pt-3 border-t border-divider'>
                    <div className='flex items-center gap-2 w-full'>
                        <Input
                            variant='bordered'
                            label={t('config.citation.search_engine')}
                            value={searchEngine}
                            onValueChange={(v) => setSearchEngine(v)}
                            className='flex-1'
                            size='sm'
                        />
                        <Dropdown>
                            <DropdownTrigger>
                                <Button variant='bordered' size='sm' className='shrink-0'>
                                    {searchEnginePresets.find((p) => p.url === searchEngine)?.label || 'Presets'}
                                </Button>
                            </DropdownTrigger>
                            <DropdownMenu
                                aria-label='search engine presets'
                                onAction={(key) => {
                                    const preset = searchEnginePresets.find((p) => p.key === key);
                                    if (preset) setSearchEngine(preset.url);
                                }}
                            >
                                {searchEnginePresets.map((p) => (
                                    <DropdownItem key={p.key}>{p.label}</DropdownItem>
                                ))}
                            </DropdownMenu>
                        </Dropdown>
                    </div>
                </div>
            </CardBody>
        </Card>
    );
}
