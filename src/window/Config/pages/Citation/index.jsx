import { DropdownTrigger } from '@nextui-org/react';
import { DropdownMenu } from '@nextui-org/react';
import { DropdownItem } from '@nextui-org/react';
import { useTranslation } from 'react-i18next';
import { CardBody } from '@nextui-org/react';
import { Dropdown } from '@nextui-org/react';
import { Switch } from '@nextui-org/react';
import { Button } from '@nextui-org/react';
import { Card } from '@nextui-org/react';
import React from 'react';

import { useConfig } from '../../../../hooks/useConfig';

export default function Citation() {
    const [windowPosition, setWindowPosition] = useConfig('citation_window_position', 'mouse');
    const [rememberWindowSize, setRememberWindowSize] = useConfig('citation_remember_window_size', false);
    const [closeOnBlur, setCloseOnBlur] = useConfig('citation_close_on_blur', true);
    const [alwaysOnTop, setAlwaysOnTop] = useConfig('citation_always_on_top', false);
    const [hideSource, setHideSource] = useConfig('citation_hide_source', false);
    const [hideLanguage, setHideLanguage] = useConfig('citation_hide_language', false);
    const [hideWindow, setHideWindow] = useConfig('citation_hide_window', false);
    const { t } = useTranslation();

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
                    <h3 className='my-auto mx-0'>{t('config.citation.hide_source')}</h3>
                    {hideSource !== null && (
                        <Switch
                            isSelected={hideSource}
                            onValueChange={(v) => {
                                setHideSource(v);
                            }}
                        />
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.citation.hide_language')}</h3>
                    {hideLanguage !== null && (
                        <Switch
                            isSelected={hideLanguage}
                            onValueChange={(v) => {
                                setHideLanguage(v);
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
            </CardBody>
        </Card>
    );
}
