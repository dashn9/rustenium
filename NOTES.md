# Browsing Contexts
Browsing Context on BiDi is the same as TargetId on CDP.
Browsing Context on BiDi AFAIK is restricted to Page & Iframe, wheareas CDP has much more.
UserContext on BiDi is different from CDP actual BrowsingContextId,
CDP BrowsingContextId appears to be consistent acrosss multiple targets, I have tried spawning a context from bidi, creating a newtab from the browser, and never got a new context via a target.

# CDP & BiDi Coexistence
CDP does not work too well alongside BiDi when both are connected simultaneously.
BiDi seems impervious to CDP usage, but CDP becomes buggy at the presence of BiDi.
However, using CDP first and then connecting BiDi afterwards does not affect the previous CDP setup.