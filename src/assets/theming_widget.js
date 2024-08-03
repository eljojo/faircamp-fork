const persistence = {
    signature: `${ACCENT_BRIGHTENING}:${ACCENT_CHROMA}:${ACCENT_HUE}:${BACKGROUND_ALPHA}:${BASE_CHROMA}:${BASE_HUE}`
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
        'accent_brightening': ACCENT_BRIGHTENING,
        'accent_chroma': ACCENT_CHROMA,
        'accent_hue': ACCENT_HUE,
        'background_alpha': BACKGROUND_ALPHA,
        'base_chroma': BASE_CHROMA,
        'base_hue': BASE_HUE
    };

    window.localStorage.setItem(
        'faircamp_theming_widget_persistence',
        JSON.stringify(persistence)
    );
}

const options = [
    {
        cssVariable: '--acc-b',
        defaultValue: 50,
        label: 'Accent Brightening',
        manifestOption: 'accent_brightening',
        range: [0, 100],
        tooltip: 'Sets the chroma (colorfulness) for accent elements (0-100%)',
        value: persistence.values.accent_brightening,
        unit: '%'
    },
    {
        cssVariable: '--acc-c',
        defaultValue: null,
        label: 'Accent Chroma',
        manifestOption: 'accent_chroma',
        range: [0, 100],
        tooltip: 'Sets the chroma (colorfulness) for accent elements (0-100%)',
        value: persistence.values.accent_chroma,
        unit: '%'
    },
    {
        cssVariable: '--acc-h',
        defaultValue: null,
        label: 'Accent Hue',
        manifestOption: 'accent_hue',
        range: [0, 360],
        tooltip: 'Sets the hue (color) for accent elements (0-360 degrees)',
        value: persistence.values.accent_hue
    },
    {
        cssVariable: '--bg-a',
        defaultValue: 10,
        label: 'Background Alpha',
        manifestOption: 'background_alpha',
        range: [0, 100],
        tooltip: 'Sets the background alpha (opaqueness) of the background image, if there is one (0-100%)',
        value: persistence.values.background_alpha,
        unit: '%'
    },
    {
        cssVariable: '--base-c',
        defaultValue: 0,
        label: 'Base Chroma',
        manifestOption: 'base_chroma',
        range: [0, 100],
        tooltip: 'Sets the base chroma (colorfulness) of the theme (0-100%)',
        value: persistence.values.base_chroma,
        unit: '%'
    },
    {
        cssVariable: '--base-h',
        defaultValue: 0,
        label: 'Base Hue',
        manifestOption: 'base_hue',
        range: [0, 360],
        tooltip: 'Sets the base hue (color) of the theme (0-360 degrees)',
        value: persistence.values.base_hue
    }
];

const message = document.createElement('div');
message.classList.add('message');

const customizations = document.createElement('textarea');

customizations.readOnly = true;

const updateTextarea = () => {
    const customized = options.filter(option => option.value !== option.defaultValue);

    if (customized.length === 0) {
        customizations.innerHTML = '';
        message.innerHTML = 'No customizations made (all options are at their default currently).';
    } else {
        customizations.innerHTML = `# theme\n\n${customized.map(option => `${option.manifestOption}: ${option.value}`).join('\n')}`;
        message.innerHTML = 'Copy the customizations below to a manifest in your catalog';
    }
};

for (const option of options) {
    const { cssVariable, defaultValue, label, manifestOption, range, tooltip, unit, value } = option;

    const valueLabel = () => `${option.value ?? 'None'}${option.value !== null && unit ? unit : ''}${option.value === defaultValue ? ' (Default)' : ''}`;

    if (value !== null) {
        const valueWithUnit = unit ? `${value}${unit}` : value.toString();
        document.querySelector(':root').style.setProperty(cssVariable, valueWithUnit);
    }

    const spanValue = document.createElement('span');

    spanValue.classList.add('value');
    spanValue.textContent = valueLabel();

    const input = document.createElement('input');

    input.min = defaultValue === null ? range[0] - 1 : range[0];
    input.max = range[1];
    input.title = tooltip;
    input.type = 'range';
    input.value = value ?? input.min;


    input.addEventListener('input', () => {
        if (defaultValue === null && input.valueAsNumber < range[0]) {
            option.value = null;
            document.querySelector(':root').style.removeProperty(cssVariable);
        } else {
            option.value = input.valueAsNumber;
            const valueWithUnit = unit ? `${option.value}${unit}` : option.value.toString();
            document.querySelector(':root').style.setProperty(cssVariable, valueWithUnit);
        }

        updateTextarea();

        spanValue.textContent = valueLabel();

        persistence.values[manifestOption] = option.value;
        persistDebounced();
    });

    const spanLabel = document.createElement('span');

    spanLabel.classList.add('label');
    spanLabel.textContent = label;
    spanLabel.title = tooltip;

    const div = document.createElement('div');

    div.classList.add('option');
    div.appendChild(spanLabel);
    div.appendChild(input);
    div.appendChild(spanValue);

    document.querySelector('.theming_widget').appendChild(div);
}

document.querySelector('.theming_widget').appendChild(message);

document.querySelector('.theming_widget').appendChild(customizations);

updateTextarea();
