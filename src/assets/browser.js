const browser = document.querySelector('#browser');
const browseButton = document.querySelector('button.browse');

const browseResults = browser.querySelector('#results');
const closeButton = browser.querySelector('button');
const searchField = browser.querySelector('input');
const statusField = browser.querySelector('[role="status"]');

const rootPrefix = browser.dataset.rootPrefix;

function truncateArtistList(artists, othersLink)  {
    const MAX_CHARS = 40;

    if (artists.length > 2) {
        const nameChars = artists.reduce((sum, artist) => sum + artist.name.length, 0);
        const separatorChars = (artists.length - 1) * 2; // All separating ", " between the artists

        if (nameChars + separatorChars > MAX_CHARS) {
            // Here we have more than two artists, we have a char limit,
            // and we cannot fit all artists within the limit, thus
            // we truncate the list.

            if (LABEL_MODE) {
                // In label mode we show at least one artist, then as many
                // additional ones as fit, e.g. "[artist],[artist] and
                // more"
                let charsUsed = 0;
                const truncatedArtists = artists
                    .filter(artist => {
                        if (charsUsed === 0) {
                            charsUsed += artist.name.length;
                            return true;
                        }

                        charsUsed += artist.name.length;
                        return charsUsed < MAX_CHARS;
                    });

                const rArtists = truncatedArtists
                    .map(artist => `<a href="${rootPrefix}${artist.url}">${artist.name}</a>`)
                    .join(", ");

                return BROWSER_JS_T.xxxAndOthers(rArtists, othersLink);
            }

            // In artist mode we show only "[catalog artist] and others".
            // Our sorting ensures the catalog artist is the first one,
            // so we can just take that.
            const rArtists = `<a href="${rootPrefix}${artists[0].url}">${artists[0].name}</a>`;

            return BROWSER_JS_T.xxxAndOthers(rArtists, othersLink);
        }
    }

    return rArtists = artists
        .map(artist => `<a href="${rootPrefix}${artist.url}">${artist.name}</a>`)
        .join(", ");
}

for (const release of RELEASES) {
    let img;
    if (release.cover) {
        img = document.createElement('img');
        img.src = rootPrefix + release.url + release.cover;
    } else {
        img = document.createElement('span');
        img.classList.add('placeholder');
    }

    const aText = document.createElement('a');
    aText.href = rootPrefix + release.url;

    const aImage = aText.cloneNode(true);
    aImage.tabIndex = -1;
    aImage.appendChild(img);

    aText.dataset.searchable = 'true';
    aText.textContent = release.title;

    const details = document.createElement('div');
    details.appendChild(aText);

    if (release.artists) {
        const artists = document.createElement('div');
        artists.classList.add('artists');
        artists.innerHTML = truncateArtistList(release.artists, `${rootPrefix}${release.url}`);
        details.appendChild(artists);
    }

    const row = document.createElement('div');
    row.appendChild(aImage);
    row.appendChild(details);
    browseResults.appendChild(row);

    for (const track of release.tracks) {
        const number = document.createElement('span');
        number.classList.add('number');
        number.textContent = track.number;

        const aTitle = document.createElement('a');
        aTitle.href = rootPrefix + track.url;

        const aImage = aTitle.cloneNode(true);
        aImage.tabIndex = -1;
        aImage.appendChild(img.cloneNode(true));

        aTitle.dataset.searchable = 'true';
        aTitle.textContent = track.title;

        const details = document.createElement('div');
        details.appendChild(number);
        details.appendChild(aTitle);

        if (track.artists) {
            const artists = document.createElement('div');
            artists.classList.add('artists');
            artists.innerHTML = truncateArtistList(track.artists, `${rootPrefix}${track.url}`);
            details.appendChild(artists);
        }

        const row = document.createElement('div');
        row.appendChild(aImage);
        row.appendChild(details);
        row.dataset.track = '';
        row.style.setProperty('display', 'none');
        browseResults.appendChild(row);
    }
}

for (const artist of ARTISTS) {
    const aText = document.createElement('a');
    aText.href = rootPrefix + artist.url;

    const imgPlaceholder = document.createElement('span');
    imgPlaceholder.classList.add('placeholder');

    const aImage = aText.cloneNode(true);
    aImage.tabIndex = -1;
    aImage.appendChild(imgPlaceholder);

    aText.dataset.searchable = 'true';
    aText.textContent = artist.name;

    const details = document.createElement('div');
    details.appendChild(aText);

    const row = document.createElement('div');
    row.appendChild(aImage);
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
    if (event.relatedTarget && !browser.contains(event.relatedTarget)) {
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
            const title = element.querySelector('[data-searchable]').textContent;
            const display = regexp.test(title);
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
