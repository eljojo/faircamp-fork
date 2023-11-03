# Download

By default your visitors can only stream your releases. There are three
mutually exclusive download modes you can enable for each release:

## Free download – `free`

For example, to enable free downloads in Opus format:

```eno
# download

free

format: opus
```

## Unlock code(s) – `code` or `codes`

An unlock code (like a coupon/token) needs to entered to access downloads.

For example, enabling FLAC and Opus downloads for people who received your download code "crowdfunding2023!" for backing you:

```eno
# download

code: crowdfunding2023!

-- unlock_text
You should have received the unlock code in your confirmation mail
for this year's crowdfunding. Stay tuned in case you missed it,
we're currently planning the next run!
-- unlock_text

formats:
- flac
- opus
```

`unlock_text` can be (optionally) used to provide a custom text to display
on the page where your visitors can enter the unlock code.

Or for example, if you have subscribers in multiple tiers, you can configure access with multiple codes:

```eno
# download

codes:
- GOLDsupporter
- SILVERsupporter

formats:
- mp3
- opus
```

## Soft Paycurtain – `price`

A soft (i.e. not technically enforced) paycurtain needs to be passed before downloading.

For example in order to ask for 4€ for accessing the FLAC downloads on a release:

```eno
# download

format: flac

price: EUR 4+
```

The `price` option accepts an [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) currency code and a price range such as:

- `USD 0+` (Name your price, including zero dollars as a valid option)
- `3.50 GBP` (Exactly 3.50 Pounds)
- `KRW 9080` (Exactly 9080 south korean won)
- `INR 230+` (230 indian rupees or more)
- `JPY 400-800` (Between 400 and 800 japanese yen)

In conjunction with this mode you will also need to specify at least one payment option, see the reference on "Payment".

## Disabled – `disabled`

Disable downloads for specific releases when they have been enabled in a manifest above in the hierarchy.

```eno
# download

disabled
```

## Offering single files for download

By default, the generated site will only offer complete download archives with
all files of a release (tracks, cover, extras) included.

Downloads of single files of a release can be enabled in addition, but be
aware that this significantly increases the required storage space for the
generated site:

```eno
# download

single_files: enabled
```

In some cases it may be preferable to *only* offer single file downloads,
entirely disabling the generation and offering of complete download archives.
This will usually require a little more space (but in some circumstances also
a little less space) than offering only complete download archives:

```eno
# download

single_files: only
```

To restore the default behavior of offering complete download archives only,
when you've overriden it in some way in a parent manifest:

```eno
# download

single_files: disabled
```

## All formats

Lastly here's a listing of all download formats you can currently enable. In
practice a minimal combination of a lossy state of the art format (e.g. `opus`),
a lossy format with high compatibility (e.g. `mp3`) and a lossless format
(e.g. `flac`) is recommended.

```eno
formats:
- aac
- aiff
- alac
- flac
- mp3
- ogg_vorbis
- opus
- opus_48
- opus_96
- opus_128
- wav
```

Note that `opus` is an alias for `opus_128`.
