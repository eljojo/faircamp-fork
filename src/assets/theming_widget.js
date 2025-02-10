const OPTIONS_MAP = {
    accentBrightening: {
        defaultValue: 50,
        key: 'accent_brightening',
        label: 'Accent Brightening',
        range: [0, 100],
        tooltip: 'Controls additional brightening for accent elements (0-100%, Default 50%)',
        unit: '%'
    },
    accentChroma: {
        defaultValue: null,
        key: 'accent_chroma',
        label: 'Accent Chroma',
        range: [0, 100],
        tooltip: 'Sets the chroma (colorfulness) for accent elements (0-100%, Default 0%)',
        unit: '%'
    },
    accentHue: {
        defaultValue: null,
        key: 'accent_hue',
        label: 'Accent Hue',
        range: [0, 360],
        tooltip: 'Sets the hue (color) for accent elements (0-360 degrees, Default 0)'
    },
    backgroundAlpha: {
        defaultValue: 10,
        key: 'background_alpha',
        label: 'Background Alpha',
        range: [0, 100],
        tooltip: 'Sets the background alpha (opaqueness) of the background image, if there is one (0-100%, Default 10%)',
        unit: '%'
    },
    base: {
        defaultValue: 'dark',
        enumValues: ['dark', 'light'],
        key: 'base',
        label: 'Base',
        range: [0, 1],
        tooltip: 'Toggle between dark and light theme base'
    },
    baseChroma: {
        defaultValue: 0,
        key: 'base_chroma',
        label: 'Base Chroma',
        range: [0, 100],
        tooltip: 'Sets the base chroma (colorfulness) of the theme (0-100%, Default 0%)',
        unit: '%'
    },
    baseHue: {
        defaultValue: 0,
        key: 'base_hue',
        label: 'Base Hue',
        range: [0, 360],
        tooltip: 'Sets the base hue (color) of the theme (0-360 degrees, Default 0)'
    },
    dynamicRange: {
        defaultValue: 0,
        key: 'dynamic_range',
        label: 'Dynamic Range',
        range: [0, 100],
        tooltip: 'Controls how far the dynamic range of this theme extends towards black (with a dark base) or white (with a light base) (0-100%, Default 0%)',
        unit: '%'
    }
};

const OPTIONS = [
    OPTIONS_MAP.accentBrightening,
    OPTIONS_MAP.accentChroma,
    OPTIONS_MAP.accentHue,
    OPTIONS_MAP.backgroundAlpha,
    OPTIONS_MAP.base,
    OPTIONS_MAP.baseChroma,
    OPTIONS_MAP.baseHue,
    OPTIONS_MAP.dynamicRange
];

const persistedJson = window.localStorage.getItem('faircamp_theming_widget_persistence');
const persisted = persistedJson ? JSON.parse(persistedJson) : null;

// Every time a new build is made, the persisted options of the theming widget
// are discarded, and build time is the marker to determine a new build has
// happened.
if (persisted && persisted.build_time === BUILD_OPTIONS.build_time) {
    for (const option of OPTIONS) {
        option.value = persisted[option.key];
    }
} else {
    for (const option of OPTIONS) {
        option.value = BUILD_OPTIONS[option.key];
    }
}

let persistTimeout = null;
function persistDebounced() {
    if (persistTimeout) { clearTimeout(persistTimeout); }

    persistTimeout = setTimeout(
        () => {
            const persistence = { build_time: BUILD_OPTIONS.build_time };
            for (const option of OPTIONS) {
                persistence[option.key] = option.value;
            }

            persistTimeout = null;
            window.localStorage.setItem(
                'faircamp_theming_widget_persistence',
                JSON.stringify(persistence)
            );
        },
        250
    );
}

const customizations = document.querySelector('.manifest');

customizations.addEventListener('mousedown', event => {
    // Middle click
    if (event.button === 1) {
        event.preventDefault();
        document.querySelector('.theming_widget').classList.toggle('advanced');
    }
});

function chromaAttenuator(lightness) {
    const RAMP = 50;
    const shape = attenuator => Math.sin(attenuator * (Math.PI / 2));
    if (lightness < RAMP) {
        const attenuator = lightness / RAMP; // 0.0 (full attenuation) - 1.0 (no attenuation)
        return shape(attenuator);
    } else if (lightness > 100 - RAMP) {
        const attenuator = (100 - lightness) / RAMP; // 1.0 (no attenuation) - 0.0 (full attenuation)
        return shape(attenuator);
    } else {
        return 1;
    }
}

function updateCssVariables() {
    const accentBrightening = OPTIONS_MAP.accentBrightening.value;
    const accentChroma = OPTIONS_MAP.accentChroma.value;
    const accentHue = OPTIONS_MAP.accentHue.value;
    const backgroundAlpha = OPTIONS_MAP.backgroundAlpha.value;
    const base = OPTIONS_MAP.base.value;
    const baseChroma = OPTIONS_MAP.baseChroma.value;
    const baseHue = OPTIONS_MAP.baseHue.value;
    const dynamicRange = OPTIONS_MAP.dynamicRange.value;

    const themeVars = base === 'dark' ? DARK_THEME : LIGHT_THEME;

    // Procedural covers are generated as png images at build time, right now
    // they only pick up the dark/light base characteristic from the theme.
    // To interactively adjust them to the theme at runtime we therefore can
    // invert them through a css filter attribute to approximate their look
    // when they are regenerated.
    document.body.classList.toggle('invert_procedural', base !== BUILD_OPTIONS['base']);

    const {
        background1LightnessRange, // based on dynamicRange
        background2LightnessRange, // based on dynamicRange
        background3LightnessRange, // based on dynamicRange
        backgroundAccentLightnessRange, // based on accentBrightening
        backgroundMiddlegroundLightnessRange, // based on dynamicRange
        foreground1FocusVariable,
        foreground1Lightness,
        foreground2Lightness,
        foreground3FocusVariable,
        foreground3Lightness,
        foregroundAccentLightness,
        foregroundMiddlegroundLightness,
        middlegroundAccentLightnessRange, // based on accentBrightening
        middlegroundLightness,
        veilAlphaRange
    } = themeVars;

    const pickFromRange = (factor, variable) => variable[0] + (factor / 100) * (variable[1] - variable[0]);

    const oklch = (l, c, h, a) => (a === undefined) ? `oklch(${l}% ${c}% ${h})` : `oklch(${l}% ${c}% ${h} / ${a}%)`;
    const set = (key, value) => document.querySelector(':root').style.setProperty(key, value);

    const background1Lightness = pickFromRange(dynamicRange, background1LightnessRange);
    const background2Lightness = pickFromRange(dynamicRange, background2LightnessRange);
    const background3Lightness = pickFromRange(dynamicRange, background3LightnessRange);
    const backgroundAccentLightness = pickFromRange(accentBrightening, backgroundAccentLightnessRange);
    const backgroundMiddlegroundLightness = pickFromRange(dynamicRange, backgroundMiddlegroundLightnessRange);
    const middlegroundAccentLightness = pickFromRange(accentBrightening, middlegroundAccentLightnessRange);
    const veilAlpha = pickFromRange(dynamicRange, veilAlphaRange);

    document.querySelector('.level[data-name="background_1"]').style.setProperty('left', `${background1Lightness}%`);
    document.querySelector('.level[data-name="background_2"]').style.setProperty('left', `${background2Lightness}%`);
    document.querySelector('.level[data-name="background_3"]').style.setProperty('left', `${background3Lightness}%`);
    document.querySelector('.level[data-name="background_middleground"]').style.setProperty('left', `${backgroundMiddlegroundLightness}%`);
    document.querySelector('.level[data-name="foreground_1"]').style.setProperty('left', `${foreground1Lightness}%`);
    document.querySelector('.level[data-name="foreground_2"]').style.setProperty('left', `${foreground2Lightness}%`);
    document.querySelector('.level[data-name="foreground_3"]').style.setProperty('left', `${foreground3Lightness}%`);
    document.querySelector('.level[data-name="foreground_middleground"]').style.setProperty('left', `${foregroundMiddlegroundLightness}%`);
    document.querySelector('.level[data-name="middleground"]').style.setProperty('left', `${middlegroundLightness}%`);

    document.querySelector('.tone .monochrome[data-name="background_1"]').style.setProperty('background', `oklch(${background1Lightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="background_2"]').style.setProperty('background', `oklch(${background2Lightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="background_3"]').style.setProperty('background', `oklch(${background3Lightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="background_accent"]').style.setProperty('background', `oklch(${backgroundAccentLightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="background_middleground"]').style.setProperty('background', `oklch(${backgroundMiddlegroundLightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="foreground_1"]').style.setProperty('background', `oklch(${foreground1Lightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="foreground_2"]').style.setProperty('background', `oklch(${foreground2Lightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="foreground_3"]').style.setProperty('background', `oklch(${foreground3Lightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="foreground_middleground"]').style.setProperty('background', `oklch(${foregroundMiddlegroundLightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="middleground"]').style.setProperty('background', `oklch(${middlegroundLightness}% 0% 0)`);
    document.querySelector('.tone .monochrome[data-name="middleground_accent"]').style.setProperty('background', `oklch(${middlegroundAccentLightness}% 0% 0)`);

    set('--bg-overlay', oklch(background1Lightness, baseChroma * chromaAttenuator(background1Lightness), baseHue, 100 - backgroundAlpha));
    set('--bg-1', oklch(background1Lightness, baseChroma * chromaAttenuator(background1Lightness), baseHue));
    set('--bg-1-90', oklch(background1Lightness, baseChroma * chromaAttenuator(background1Lightness), baseHue, 90));
    set('--bg-1-overlay', oklch(background1Lightness, baseChroma * chromaAttenuator(background1Lightness), baseHue, 80));
    set('--bg-2', oklch(background2Lightness, baseChroma * chromaAttenuator(background2Lightness), baseHue));
    set('--bg-2-overlay', oklch(background2Lightness, baseChroma * chromaAttenuator(background2Lightness), baseHue, 80));
    set('--bg-3', oklch(background3Lightness, baseChroma * chromaAttenuator(background3Lightness), baseHue));
    set('--bg-acc', oklch(backgroundAccentLightness, accentChroma !== null ? accentChroma : baseChroma * chromaAttenuator(backgroundAccentLightness), accentHue ?? baseHue));
    set('--bg-acc-overlay', oklch(backgroundAccentLightness, accentChroma !== null ? accentChroma : baseChroma * chromaAttenuator(backgroundAccentLightness), accentHue ?? baseHue, 80));
    set('--bg-mg', oklch(backgroundMiddlegroundLightness, baseChroma * chromaAttenuator(backgroundMiddlegroundLightness), baseHue));
    set('--fg-1', oklch(foreground1Lightness, baseChroma * chromaAttenuator(foreground1Lightness), baseHue));
    set('--fg-1-focus', `var(${foreground1FocusVariable})`);
    set('--fg-1-veil', oklch(foreground1Lightness, 0, 0, veilAlpha));
    set('--fg-2', oklch(foreground2Lightness, baseChroma * chromaAttenuator(foreground2Lightness), baseHue));
    set('--fg-3', oklch(foreground3Lightness, baseChroma * chromaAttenuator(foreground3Lightness), baseHue));
    set('--fg-3-focus', `var(${foreground3FocusVariable})`);
    set('--fg-acc', oklch(foregroundAccentLightness, 0, 0));
    set('--fg-mg', oklch(foregroundMiddlegroundLightness, baseChroma * chromaAttenuator(foregroundMiddlegroundLightness), baseHue));
    set('--mg', oklch(middlegroundLightness, baseChroma * chromaAttenuator(middlegroundLightness), baseHue));
    set('--mg-acc', oklch(middlegroundAccentLightness, accentChroma !== null ? accentChroma : baseChroma * chromaAttenuator(middlegroundAccentLightness), accentHue ?? baseHue));
    set('--mg-acc-overlay', oklch(middlegroundAccentLightness, accentChroma !== null ? accentChroma : baseChroma * chromaAttenuator(middlegroundAccentLightness), accentHue ?? baseHue, 80));

    set('--fg-1-theming-widget', oklch(foreground1Lightness, 0, 0));
    set('--fg-2-theming-widget', oklch(foreground2Lightness, 0, 0));
    set('--bg-1-theming-widget', oklch(background1Lightness, 0, 0));
    set('--bg-1-overlay-theming-widget', oklch(background1Lightness, 0, 0, 90));
    set('--bg-2-overlay-theming-widget', oklch(background2Lightness, 0, 0, 90));
    set('--mg-theming-widget', oklch(middlegroundLightness, 0, 0, 90));
}

function updateState() {
    updateCssVariables();
    updateTextarea();
}

function updateTextarea() {
    const customized = OPTIONS.filter(option => option.value !== option.defaultValue);

    let message = '';
    if (customized.length === 0) {
        message += 'Tip:\n'
        message += '- Middle-click a slider to reset its value\n'
        message += '- Middle-click the area that contains this text for advanced tooling\n'
    } else {
        message += '> Copy these customizations to a manifest in your catalog\n';
        message += 'theme:\n'
        message += customized.map(option => `${option.key} = ${option.value}`).join('\n');
    }
    customizations.value = message;
}

for (const option of OPTIONS) {
    const { defaultValue, enumValues, label, key, range, tooltip, unit, value } = option;

    const valueLabel = () => `${option.value ?? 'None'}${option.value !== null && unit ? unit : ''}${option.value === defaultValue ? ' (Default)' : ''}`;

    const spanValue = document.createElement('span');

    spanValue.classList.add('value');
    spanValue.textContent = valueLabel();

    const input = document.createElement('input');

    input.min = defaultValue === null ? range[0] - 1 : range[0];
    input.max = range[1];
    input.title = tooltip;
    input.type = 'range';

    if (enumValues) {
        input.value = enumValues.indexOf(value);
    } else {
        input.value = value ?? input.min;
    }

    input.addEventListener('mousedown', event => {
        // Middle click
        if (event.button === 1) {
            event.preventDefault();
            option.value = defaultValue;

            if (enumValues) {
                input.value = enumValues.indexOf(option.value);
            } else {
                input.value = option.value ?? input.min;
            }

            updateState();

            spanValue.textContent = valueLabel();

            persistDebounced();
        }
    });

    input.addEventListener('input', () => {
        if (enumValues) {
            option.value = enumValues[input.valueAsNumber];
        } else if (input.valueAsNumber < range[0]) {
            option.value = null;
        } else {
            option.value = input.valueAsNumber;
        }

        updateState();

        spanValue.textContent = valueLabel();

        persistDebounced();
    });

    const spanLabel = document.createElement('span');

    spanLabel.classList.add('label');
    spanLabel.textContent = label;
    spanLabel.title = tooltip;

    const div = document.createElement('div');

    div.classList.add('option');
    div.dataset.name = key;
    div.appendChild(spanLabel);
    div.appendChild(input);
    div.appendChild(spanValue);

    document.querySelector('.theming_widget .controls').appendChild(div);
}

updateState();
