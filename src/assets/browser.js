const browser = document.querySelector('#browser');
const browseButton = document.querySelector('button#browse');

const browseResults = browser.querySelector('#results');
const closeButton = browser.querySelector('button');
const searchField = browser.querySelector('input');
const statusField = browser.querySelector('[role="status"]');

const rootPrefix = browser.dataset.rootPrefix;

for (const release of RELEASES) {
    const img = document.createElement('img');
    img.ariaHidden = 'true';
    img.src = rootPrefix + release.url + release.cover;

    const a = document.createElement('a');
    a.href = rootPrefix + release.url;
    a.textContent = release.title;

    const details = document.createElement('div');
    details.appendChild(a);

    if (release.artists) {
        const artists = document.createElement('div');

        artists.classList.add('artists');

        for (const artist of release.artists) {
            const a = document.createElement('a');
            a.href = rootPrefix + artist.url;
            a.textContent = artist.name;
            artists.appendChild(a);
        }

        details.appendChild(artists);
    }

    const row = document.createElement('div');
    row.appendChild(img);
    row.appendChild(details);
    browseResults.appendChild(row);

    for (const track of release.tracks) {
        const img = document.createElement('img');
        img.ariaHidden = 'true';
        img.src = rootPrefix + release.url + release.cover;

        const number = document.createElement('span');
        number.classList.add('number');
        number.textContent = track.number;

        const title = document.createElement('a');
        title.href = rootPrefix + track.url;
        title.textContent = track.title;

        const details = document.createElement('div');
        details.appendChild(number);
        details.appendChild(title);

        if (track.artists) {
            const artists = document.createElement('div');

            artists.classList.add('artists');

            for (const artist of track.artists) {
                const a = document.createElement('a');
                a.href = rootPrefix + artist.url;
                a.textContent = artist.name;
                artists.appendChild(a);
            }

            details.appendChild(artists);
        }

        const row = document.createElement('div');
        row.appendChild(img);
        row.appendChild(details);
        row.dataset.track = '';
        row.style.setProperty('display', 'none');
        browseResults.appendChild(row);
    }
}

for (const artist of ARTISTS) {
    const imgPlaceholder = document.createElement('span');
    imgPlaceholder.ariaHidden = 'true';
    imgPlaceholder.classList.add('placeholder');

    const a = document.createElement('a');
    a.href = rootPrefix + artist.url;
    a.textContent = artist.name;

    const details = document.createElement('div');
    details.appendChild(a);

    const row = document.createElement('div');
    row.appendChild(imgPlaceholder);
    row.appendChild(details);
    browseResults.appendChild(row);
}

function hideBrowser() {
    browser.classList.remove('active');
    browseButton.setAttribute('aria-expanded', 'false');
    searchField.value = '';
    statusField.removeAttribute('aria-label');
    statusField.textContent = '';
    for (const result of browseResults.children) {
        const display = result.dataset.track === undefined;
        result.style.setProperty('display', display ? null : 'none');
    }
    browseButton.focus();
}

function showBrowser() {
    browser.classList.add('active');
    browseButton.setAttribute('aria-expanded', 'true');
    searchField.focus();
    statusField.setAttribute('aria-label', BROWSER_JS_T.showingFeaturedItems);
    statusField.textContent = '';
}

browser.addEventListener('focusout', event => {
    if (!event.relatedTarget || !browser.contains(event.relatedTarget)) {
        hideBrowser();
    }
});

browser.addEventListener('keydown', event => {
    if (event.key === 'Escape') {
        event.preventDefault();
        hideBrowser();
    }
});

browseButton.addEventListener('click', () => {
    if (browser.classList.contains('active')) {
        hideBrowser();
    } else {
        showBrowser();
    }
});

closeButton.addEventListener('click', hideBrowser);

searchField.addEventListener('input', () => {
    const query = searchField.value.trim();

    if (query.length) {
        const regexp = new RegExp(query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'i');
        let shown = 0;

        for (const element of browseResults.children) {
            const display = regexp.test(element.querySelector('a').textContent);
            element.style.setProperty('display', display ? null : 'none');
            if (display) { shown += 1; }
        }

        if (shown === 0) {
            statusField.removeAttribute('aria-label');
            statusField.textContent = BROWSER_JS_T.nothingFoundForXxx(query);
        } else {
            statusField.setAttribute('aria-label', BROWSER_JS_T.showingXxxResultsForXxx(shown, query));
            statusField.textContent = '';
        }
    } else {
        for (const element of browseResults.children) {
            const display = element.dataset.track === undefined;
            element.style.setProperty('display', display ? null : 'none');
        }

        statusField.setAttribute('aria-label', BROWSER_JS_T.showingFeaturedItems);
        statusField.textContent = '';
    }
});
