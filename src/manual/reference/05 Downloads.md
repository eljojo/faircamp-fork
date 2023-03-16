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
You should have received the unlock code in your confirmation mail for this year's crowdfunding. Stay tuned in case you missed it, we're currently planning the next run!
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

## All formats

Lastly here's a listing of all download formats you can currently enable. In
practice a minimal lossless/lossy combination is recommended, e.g. `flac` and
`opus`. Note that `opus` is an alias for `opus_128`.

```eno
formats:
- aac
- aiff
- flac
- mp3
- ogg_vorbis
- opus
- opus_48
- opus_96
- opus_128
- wav
```