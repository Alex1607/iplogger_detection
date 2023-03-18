# IP Logger Detection

This Couldflare Worker is able to detect IP Loggers even if they are hidden behind link shorteners like bit.ly.
It will follow HTTP Redirects until it finds an IPLogger and then return the result.

The IPLogger URLs are from the FastForwardTeam: https://github.com/FastForwardTeam/FastForward/blob/main/src/js/rules.json#L315-L375

