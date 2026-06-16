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
import React, { useEffect } from 'react';

import { useConfig } from '../../../../hooks/useConfig';
import { invoke } from '@tauri-apps/api';

export default function Download() {
    const [downloadDir, setDownloadDir] = useConfig('download_dir', '');
    const [autoDownloadCount, setAutoDownloadCount] = useConfig('download_auto_count', '0');
    const [autoOpenPdf, setAutoOpenPdf] = useConfig('download_auto_open', false);
    const [autoOpenDoi, setAutoOpenDoi] = useConfig('download_auto_open_doi', false);
    const [cfHost, setCfHost] = useConfig('citation_cf_host', '');
    const [cfPort, setCfPort] = useConfig('citation_cf_port', '');
    const [cfStatus, setCfStatus] = React.useState('');
    const { t } = useTranslation();

    // Sync CF bypass config to backend on change
    useEffect(() => {
        if (cfHost === null || cfPort === null) return;
        const port = parseInt(cfPort, 10) || 8000;
        invoke('update_cf_config', { host: cfHost || '127.0.0.1', port });
    }, [cfHost, cfPort]);

    return (
        <Card>
            <CardBody>
                <div className='config-item mt-4 pt-3 border-t border-divider'>
                    <div className='flex items-center gap-2 w-full'>
                        <Input
                            variant='bordered'
                            label={t('config.download.download_dir')}
                            value={downloadDir}
                            onValueChange={(v) => setDownloadDir(v)}
                            className='flex-1'
                            size='sm'
                        />
                    </div>
                </div>
                <div className='config-item mt-4 pt-3 border-t border-divider'>
                    <h3 className='my-auto mx-0 shrink-0 whitespace-nowrap pr-2'>{t('config.download.cf_bypass')}</h3>
                    <div className='flex items-center gap-2 w-full'>
                        <Input
                            variant='bordered'
                            label={t('config.download.cf_host')}
                            placeholder='127.0.0.1'
                            value={cfHost}
                            onValueChange={(v) => setCfHost(v)}
                            className='flex-1'
                            size='sm'
                        />
                        <Input
                            variant='bordered'
                            label={t('config.download.cf_port')}
                            placeholder='8000'
                            value={cfPort}
                            onValueChange={(v) => setCfPort(v)}
                            className='w-24'
                            size='sm'
                        />
                        <Button
                            size='sm'
                            variant='flat'
                            onPress={async () => {
                                const port = parseInt(cfPort, 10) || 8000;
                                try {
                                    const result = await invoke('test_cf_bypass', { host: cfHost || '127.0.0.1', port });
                                    setCfStatus(result ? t('config.download.cf_test_ok') : t('config.download.cf_test_fail'));
                                } catch {
                                    setCfStatus(t('config.download.cf_test_fail'));
                                }
                            }}
                        >
                            {t('config.download.cf_test')}
                        </Button>
                    </div>
                    {cfStatus && (
                        <div className='text-tiny text-default-500 mt-1 whitespace-pre-wrap'>
                            {cfStatus}
                        </div>
                    )}
                </div>
                <div className='config-item mt-4 pt-3 border-t border-divider'>
                    <h3 className='my-auto mx-0 shrink-0 whitespace-nowrap pr-2'>{t('config.download.auto_download_count')}</h3>
                    <Input
                        variant='bordered'
                        type='number'
                        value={autoDownloadCount}
                        onValueChange={(v) => setAutoDownloadCount(v)}
                        className='w-24'
                        size='sm'
                        min={0}
                    />
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.download.auto_open_pdf')}</h3>
                    {autoOpenPdf !== null && (
                        <Switch isSelected={autoOpenPdf} onValueChange={(v) => setAutoOpenPdf(v)} />
                    )}
                </div>
                <div className='config-item'>
                    <h3 className='my-auto mx-0'>{t('config.download.auto_open_doi')}</h3>
                    {autoOpenDoi !== null && (
                        <Switch isSelected={autoOpenDoi} onValueChange={(v) => setAutoOpenDoi(v)} />
                    )}
                </div>
            </CardBody>
        </Card>
    );
}
