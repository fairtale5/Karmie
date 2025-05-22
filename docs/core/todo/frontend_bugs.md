Main issue everywhere:
We do NOT want to display idiot assumptions in code. If the auth state is not loaded, we display a loading state, we do NOT display "not logged in" assumptions, not even temporarily while the auth state is loading.

/onboarding page:
- [ ] Remove "warning" that displays for a second until the user auth is initialized. Display 'loading' instead and put a placeholder loading pulse effect on the set up your profile component during that time.
- [ ] "You must be logged in to set up your profile" inside the set up your profile component should not be there at all.

/profile page:
- [ ] "please sign in" warning is still there while auth state is loading. It should not be there.
- [ ] currrently only displays auth info (principal, created date), but doesn't include user info from the user document. Should change it so the header section displays basic juno info (principal), and then bellow it loads the user document info.

