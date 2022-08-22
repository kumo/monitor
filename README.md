# monitor
A microsite that sends a Telegram message when a server is offline.

There are two links: a `/hello` that is called from the server to be monitor, and a `/check/MINUTES` that is called on the microsite.

The `/check/MINUTES` url expects to be passed a number that represents the number of minutes that need to pass before the monitor sends a message.

Currently, the time is saved in a file, but another solution would be needed for a service such as Heroku.
