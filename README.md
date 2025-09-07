# guest-list-qrs

I'm getting married in 2027 - of COURSE I had to pull a classic developer move and make some kind of app to automate my wedding planning process!

This is a guest list QR code generator built with rust &amp; an integration for google sheets.

## Requires

[Rust & Cargo]("https://www.rust-lang.org/tools/install")

## Setup

Make a local copy of `.env.example` called `.env`. This is where you will define the url to link to the QR code.

Make a local copy of `.tokens.example.csv` called `.tokens.csv`. This is where you define your unique tokens or ids for different urls.
(Note: You don't HAVE to call it `.tokens.csv` but if you do change the name make sure to update the path in the `.env` file to match correctly)

## Build & Run

`cargo build` then `cargo run`

## What does it do?

When you run the app, it reads the base URL from `.env` and each token from `.tokens.csv`, concatonates those two values to build a unique url, then generates a QR code for each token that links to that url. The app creates an `/out` directory (if it doesn’t already exist) and saves each QR code as an svg file named after its token value. Please remember to make sure the tokens are unique to avoid overwriting files!

### Example Flow

1. Assign each guest a unique token/ID (e.g., 123, 456).

2. Prepare your RSVP page to accept a URL parameter (e.g., ?guestid=).

3. Create `.tokens.csv` in the project root and list all unique guest tokens (e.g., 123,456,678,876).

4. Run the app to generate an `/out` dir and create each QR code saved in this dir as an SVG named after its token.

5. Print invitations with each guest’s unique QR code.

6. Guests scan their QR code and are directed to their personalised RSVP page.

### Example output

```
Generating QR codes...
Skipped invalid token: 152##726
Skipped invalid token: 15@$@$2727
Successfully generated QR codes. Files are in the /out directory. Enjoy!
```

### Example error.log

```
----- 07/09/2025 14:39:18 -----
Invalid token '152##726': token must be alphanumeric
Invalid token '15@$@$2727': token must be alphanumeric
```
