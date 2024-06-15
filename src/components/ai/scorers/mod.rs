// These are special components that
// run in the background, calculating a "Score" value, which is what Big Brain
// will use to pick which Actions to execute.
//
// Just like with Actions, there is a distinction between Scorer components
// and the ScorerBuilder which will attach those components to the Actor entity.
//
// Again, in most cases, you can use the `ScorerBuilder` derive macro to make your
// Scorer Component act as a ScorerBuilder. You need it to implement Clone and Debug.

pub(crate) mod thirsty;
