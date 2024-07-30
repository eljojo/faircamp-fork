const persistence = {
    signature: `${ACCENT_CHROMA}:${ACCENT_HUE}:${BACKGROUND_CHROMA}:${BACKGROUND_HUE}:${LINK_H}:${LINK_S}:${TEXT_H}:${TINT_FRONT}`
};

let persistTimeout = null;
function persistDebounced() {
    if (persistTimeout) { clearTimeout(persistTimeout); }

    persistTimeout = setTimeout(
        () => {
            persistTimeout = null;
            window.localStorage.setItem(
                'faircamp_theming_widget_persistence',
                JSON.stringify(persistence)
            );
        },
        250
    );
}

const persistedJson = window.localStorage.getItem('faircamp_theming_widget_persistence');

if (persistedJson) {
    const persisted = JSON.parse(persistedJson);
    if (persistence.signature === persisted.signature) {
        persistence.values = persisted.values; // restore previous values
    }
}

if (!persistence.values) {
    // A new build with differing values, or we never persisted before
    persistence.values = {
        'accent_chroma': ACCENT_CHROMA,
        'accent_hue': ACCENT_HUE,
        'background_chroma': BACKGROUND_CHROMA,
        'background_hue': BACKGROUND_HUE,
        'link_hue': LINK_H,
        'link_saturation': LINK_S,
        'text_hue': TEXT_H,
        'tint_front': TINT_FRONT
    };

    window.localStorage.setItem(
        'faircamp_theming_widget_persistence',
        JSON.stringify(persistence)
    );
}

const options = [
    {
        cssVariable: '--acc-c',
        default: 0,
        label: 'Accent Chroma',
        manifestOption: 'accent_chroma',
        range: [0, 0.37],
        step: 'any',
        tooltip: 'Sets the chroma (colorfulness) of the page accent (0-0.37)',
        value: persistence.values.accent_chroma
    },
    {
        cssVariable: '--acc-h',
        default: 0,
        label: 'Accent Hue',
        manifestOption: 'accent_hue',
        range: [0, 360],
        step: 1,
        tooltip: 'Sets the hue of the accent (0-360 degrees)',
        value: persistence.values.accent_hue
    },
    {
        cssVariable: '--bg-c',
        default: 0,
        label: 'Background Chroma',
        manifestOption: 'background_chroma',
        range: [0, 0.37],
        step: 'any',
        tooltip: 'Sets the chroma (colorfulness) of the page background (0-0.37)',
        value: persistence.values.background_chroma
    },
    {
        cssVariable: '--bg-h',
        default: 0,
        label: 'Background Hue',
        manifestOption: 'background_hue',
        range: [0, 360],
        step: 1,
        tooltip: 'Sets the hue of the background (0-360 degrees)',
        value: persistence.values.background_hue
    },
    {
        cssVariable: '--link-h',
        default: 0,
        label: 'Link Hue',
        manifestOption: 'link_hue',
        range: [0, 360],
        step: 1,
        tooltip: 'Sets the hue of the link color (0-360 degrees)',
        value: persistence.values.link_hue,
        unit: 'deg'
    },
    {
        cssVariable: '--link-s',
        default: 0,
        label: 'Link Saturation',
        manifestOption: 'link_saturation',
        range: [0, 100],
        step: 1,
        tooltip: 'Sets the saturation of the link color (0-100 percent)',
        value: persistence.values.link_saturation,
        unit: '%'
    },
    {
        cssVariable: '--tint-front',
        default: 0,
        label: 'Text Color Tint',
        manifestOption: 'tint_front',
        range: [0, 100],
        step: 1,
        tooltip: 'Applies a color tint to the page foreground elements (0-100%)',
        value: persistence.values.tint_front
    },
    {
        cssVariable: '--text-h',
        default: 0,
        label: 'Text Hue',
        manifestOption: 'text_hue',
        range: [0, 360],
        step: 1,
        tooltip: 'Sets the hue of the text color (0-360 degrees)',
        value: persistence.values.text_hue,
        unit: 'deg'
    }
];

const message = document.createElement('div');
message.classList.add('message');

const customizations = document.createElement('textarea');

customizations.readOnly = true;

const updateTextarea = () => {
    const customized = options.filter(option => option.value !== option.default);

    if (customized.length === 0) {
        customizations.innerHTML = '';
        message.innerHTML = 'No customizations made (all options are at their default currently).';
    } else {
        customizations.innerHTML = `# theme\n\n${customized.map(option => `${option.manifestOption}: ${option.value}`).join('\n')}`;
        message.innerHTML = 'Copy the customizations below to a manifest in your catalog';
    }
};

for (const option of options) {
    const { cssVariable, label, manifestOption, range, step, tooltip, unit, value } = option;

    const valueWithUnit = unit ? `${value}${unit}` : value.toString();
    document.querySelector(':root').style.setProperty(cssVariable, valueWithUnit);

    const input = document.createElement('input');

    if (range) {
        input.min = range[0];
        input.max = range[1];
    }

    input.step = step;
    input.title = tooltip;
    input.type = 'range';
    input.value = value;

    input.addEventListener('input', () => {
        option.value = input.valueAsNumber;

        const valueWithUnit = unit ? `${option.value}${unit}` : option.value.toString();
        document.querySelector(':root').style.setProperty(cssVariable, valueWithUnit);

        updateTextarea();

        persistence.values[manifestOption] = option.value;
        persistDebounced();
    });

    const span = document.createElement('span');

    span.classList.add('label');
    span.innerHTML = label;
    span.title = tooltip;

    const div = document.createElement('div');

    div.classList.add('option');
    div.appendChild(input);
    div.appendChild(span);

    document.querySelector('.theming_widget').appendChild(div);
}

document.querySelector('.theming_widget').appendChild(message);

document.querySelector('.theming_widget').appendChild(customizations);

updateTextarea();
