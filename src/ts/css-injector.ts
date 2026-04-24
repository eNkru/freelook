// CSS Injector - Tauri version
// Provides CSS strings for ad blocking and frameless window styling

export function getMainCss(config: Record<string, string>): string {
    const verticalAdsClass = config.verticalAdsClass || 'pBKjV';
    const smallAdsClass = config.smallAdsClass || 'X1Kvq';
    const premiumAdsClass = config.premiumAdsClass || 'VPtFl';

    return `
        .${verticalAdsClass} { display: none !important; }
        .${smallAdsClass} { display: none !important; }
        .${premiumAdsClass} { display: none !important; }
    `;
}

export const NO_FRAME_CSS = `
    ._1Kg3ffZABPxXxDqcmoxkBA {
        padding-top: 30px !important;
        -webkit-app-region: drag;
    }
    .ms-FocusZone,
    ._3Nd2PGu67wifhuPZp2Sfj5 {
        -webkit-app-region: no-drag;
    }
`;