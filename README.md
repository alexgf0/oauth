This repository implements a basic Rust server that uses OAuth2 to obtain the gmail from a Google user.
The implementation avoids the use of frameworks(regarding OAuth functionality) to exemplify the minimum basic OAuth workflow.

For purposes of brevity, neither tests nor error handling has been implemented.

# Interesting references
[OAuth workflow by IBM](https://www.ibm.com/docs/en/tfim/6.2.2.6?topic=overview-oauth-20-workflow)

[Google's good documentation](https://developers.google.com/identity/protocols/oauth2/web-server#httprest_1)


# Steps to get google user data
## Enabling OAuth from your Google account
[OAuth Google Configuration](https://console.cloud.google.com/apis/dashboard)

## Get user authorization code

## Get access token using the authorization code
we had problems with the timing and reuse of the authorization code given...


## Use the access tokens to get google's user information


