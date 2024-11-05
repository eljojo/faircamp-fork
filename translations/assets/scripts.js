const PERSISTENCE_KEY = 'faircampTranslateState';

const clearButton = document.querySelector('button#clear');
const headerCountSpan = document.querySelector('header span.count');
const messageSpan = document.querySelector('span.message');
const messageTextSpan = document.querySelector('span.message .text');
const submissionDiv = document.querySelector('div.submission');

let scheduledActivityOff = null;
let scheduledPersist = null;
let state = {};

function clearTranslations() {
    for (const language of document.querySelectorAll('[data-language-code]')) {
        for (const element of language.querySelectorAll('[data-translation-key]')) {
            element.value = '';
        }
        const languageCountSpan = language.parentElement.querySelector('.count');
        languageCountSpan.classList.remove('active');
        languageCountSpan.textContent = '';
    }
    state = {};
    persist();
    updateControls();
    updateSubmission();
    message('Cleared all changes from your session');
}

function countTranslations() {
    return Object.values(state)
        .reduce((count, translations) =>(count + Object.values(translations).length), 0)
}

function message(text) {
    messageTextSpan.textContent = text;

    if (scheduledActivityOff !== null) {
        clearTimeout(scheduledActivityOff);
    } else {
        messageSpan.classList.add('active');
    }

    scheduledActivityOff = setTimeout(
        () => {
            messageSpan.classList.remove('active');
            scheduledActivityOff = null;
        },
        5000
    );
}

function persist() {
    const stateSerialized = JSON.stringify(state);
    localStorage.setItem(PERSISTENCE_KEY, stateSerialized);
    message(`Saved all ${countTranslations()} changes (to browser localStorage)`);
}

function persistDebounced() {
    if (scheduledPersist !== null) {
        clearTimeout(scheduledPersist);
    }

    scheduledPersist = setTimeout(
        () => {
            persist();
            scheduledPersist = null;
        },
        200
    );
}

function restore() {
    const stateSerialized = localStorage.getItem(PERSISTENCE_KEY);
    if (stateSerialized) {
        try {
            state = JSON.parse(stateSerialized);

            for (const [languageCode, translations] of Object.entries(state)) {
                for (const [translationKey, value] of Object.entries(translations)) {
                    const element = document.querySelector(`[data-language-code="${languageCode}"] [data-translation-key="${translationKey}"]`);
                    if (element) {
                        element.value = value;
                    } else {
                        delete state[languageCode][translationKey];
                        if (!Object.values(state[languageCode]).length) {
                            delete state[languageCode];
                        }
                    }
                }

                if (languageCode in state) {
                    const languageCountSpan = document.querySelector(`[data-language-code="${languageCode}"]`).parentElement.querySelector('.count');
                    languageCountSpan.classList.add('active');
                    languageCountSpan.textContent = `${Object.values(state[languageCode]).length} changes`;
                }
            }

            if (countTranslations()) {
                updateControls();
                updateSubmission();
                message(`Restored all ${countTranslations()} changes from a previous session (from browser localStorage)`);
                return;
            }
        } catch { /* pass */ }
    }

    message(`New, empty session started`);
}

function updateControls() {
    function disableControls(disable) {
        clearButton.disabled = disable;
    }

    const count = countTranslations();

    headerCountSpan.textContent = `${count} changes`;

    if (count) {
        disableControls(false);
    } else {
        disableControls(true);
    }
}

function updateSubmission() {
    const languages = [];

    for (const [languageCode, translations] of Object.entries(state)) {
        let language = `${languageCode.toUpperCase()} {\n`;

        const strings = [];
        for (const [translationKey, value] of Object.entries(translations)) {
            // TODO: Escape
            strings.push(`    ${translationKey}: "${value}"`);
        }
        language += strings.join(',\n') + '\n';

        language += '};\n';

        languages.push(language);
    }

    const submission = languages.join('\n');

    const pre = document.createElement('pre');

    pre.textContent = submission;

    submissionDiv.replaceChildren(pre);
}

function updateTranslation(languageCode, translationKey, value) {
    const trimmedValue = value.trim();

    if (trimmedValue.length) {
        state[languageCode] ||= {};
        state[languageCode][translationKey] = trimmedValue;
    } else {
        if (languageCode in state) {
            delete state[languageCode][translationKey];
            if (!Object.values(state[languageCode]).length) {
                delete state[languageCode];
            }
        }
    }

    const languageCountSpan = document.querySelector(`[data-language-code="${languageCode}"]`).parentElement.querySelector('.count');
    if (languageCode in state) {
        languageCountSpan.classList.add('active');
        languageCountSpan.textContent = `${Object.values(state[languageCode]).length} changes`;
    } else {
        languageCountSpan.classList.remove('active');
        languageCountSpan.textContent = '';
    }

    persistDebounced();
    updateControls();
    updateSubmission();
}

clearButton.addEventListener('click', () => {
    if (confirm(`THIS WILL DISCARD ALL ${countTranslations()} TRANSLATIONS YOU ENTERED\n\nREALLY CLEAR?`)) {
        clearTranslations();
    }
});

for (const language of document.querySelectorAll('[data-language-code]')) {
    for (const element of language.querySelectorAll('[data-translation-key]')) {
        element.addEventListener('input', () =>
            updateTranslation(language.dataset.languageCode, element.dataset.translationKey, element.value)
        );
    }
}

restore();
