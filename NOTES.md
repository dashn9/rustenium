# Browsing Contexts
Browsing Context on BiDi is the same as TargetId on CDP.
Browsing Context on BiDi AFAIK is restricted to Page & Iframe, wheareas CDP has much more.
UserContext on BiDi is different from CDP actual BrowsingContextId,
CDP BrowsingContextId appears to be consistent acrosss multiple targets, I have tried spawning a context from bidi, creating a newtab from the browser, and never got a new context via a target.