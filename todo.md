# Things that need to happen

## High priority
- [x] fix collision with elevator to make it not wonky when jumping - not a great solution
- [x] hop when approaching an elevator
- [x] change the elevators to know which floor number they are on (ie. use a start number/offset)
- [x] door logic for drawing doors in correct places
- [x] open doors when user is nearby and facing and they have documents
- [x] reorganize the packages to improve compilation and organization
- [x] draw the empty room behind it
- [x] "parallax" movement of camera
- [x] draw out a fuller map
- [x] add a frame rate counter
- [x] draw elevators in proper places based on tiles
- [x] render elevator shafts as a sprite
- [x] render the overlays separately to put at a higher z
- [x] player falls off the elevator when going up in elevator
- [x] player still bounces on elevator
- [x] don't "overjump" when elevator is going up
- [x] use a resource to keep track of which levels are drawn
- [x] don't draw offscreen collision layers, doors, lamps, elevators
- [x] remove offscreen collision layers, doors, lamps, elevators
- [x] draw all elevators
- [ ] escalator logic and movement
- [ ] animate the user going into the room and coming out
- [ ] mark room as used when user comes out
- [ ] create a system for knowing when the player is near an elevator (or on an elevator)
- [ ] don't move elevators unless user is inside (ie. can't control from the top)
- [ ] add bad guys and AI
- [ ] door entry draws on the wrong side depending on the direction

## Medium priority
- [x] fix gun transformations so that it looks correct
- [x] fix elevator so that it waits
- [ ] elevators can go above their boundaries when controlling (hold up arrow key) - only some elevators
- [ ] draw lamps above doors as part of door rendering
- [ ] lamps as sprites, ie. falling and lights out
- [ ] draw elevator cable
- [ ] draw tiles only when needed as well (remove offscreen)
- [ ] draw properly colored elevator overlay bottoms and tops depending on color or all the way at the top
- [ ] still not always 100% accurate with crossing between elevator and floor

## Low priority
- [ ] increase user velocity when riding up elevator so jump is additional
- [ ] draw floor numbers
- [ ] animate intro
- [ ] scoring
- [ ] Ducking when on top of elevator
- [ ] add sounds
- [ ] add music
- [ ] adjust door entry mechanism so he doesn't open it too early
- [ ] two player game
- [ ] add an EntityLoader type of trait to move stuff out of map
- [ ] it's possible to shoot "through" walls if the player has his arm outside
