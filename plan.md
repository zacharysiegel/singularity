## Title

Ideas
    Singularity
    Neural Foundry
    Silicogenesis

## Main idea

Genre: Slow-tick RTS
Core mechanic: Factory automation
Graphics: 2D

## Geography

Map
    Dense hex board
    Resource nodes
    Empty nodes
        Collectors fill resource nodes
        Buildings fill empty nodes
        Transportation paths must pass through empty nodes
    Generation
        Resource clusters
            Strategy 1
                Each hex is seeded with a starting weight (e.g. 10)
                That weight's proportion of the total is the probability of centering a cluster on that hex
                Once a cluster is centered on a hex, the rest of that cluster can be placed around it with minimal restriction
                Once a cluster is placed, the weights in its neighborhood should be modified to reflect the new state
                    Immediate neighbors should be set to zero until a minimum distance is reached
                    After the minimum distance is set, there should be an intermediate zone where the weights are incrementally diminished
                    There should be a maximum distance after which there is no effect on new weights
                    There must be different weights for each resource type
                    Effects on weights across different resource types should be less significant than within the same type
            Strategy 2
                For n resource clusters and a R2 plane projected atop the hex plane
                Initialize random coordinates for each resource cluster
                Simulate each resource as a charged particle (or similar)
                    Each charge, weighted by distance, induces a force on each other particle
                    Particles are accelerated + moved in time step simulation
                    Terminate after k steps or minimum total movement threshold or minimum total force threshold reached
                    Problems at the edges of map can be solved by actually running the particle simulation on a larger map (x/y + average distance between particles), then truncating to actual size
                Map back to discrete hex locations
                Half particles are oil, half are metal
                    Two-coloring problem?

## Economy

Resource extraction
    Energy
        Collected in large quantities at specific geothermal points
        Collected in small quantities solar panels anywhere
    Material
        Collected by mines at specific locations
    Veins
        Forces conflict between adjoining players
        Enforces single player control over vein

Resource transportation
    No transportation infractructure (Subterfuge-like)
    Transportation is committed after send
    If target is destroyed, items accumulate at the ruins
    Resources can be intercepted by opponents

Overclocking
    Consume more energy in order to increase speed
    Energy cost is significantly less efficient than horizontal scaling
    Any machine (e.g. mine, belt, assembler) can be overclocked
    Energy-rich players should prefer to trade rather than overclock

Assembly/Crafting
    Items can be created via the conjunction of other items
    All assemply is performed by assembler objects
        There is no "manual" player assembly
    Should assembly be instantaneous?
    Assemblers can be constructed to automatically assemble items
        Un-differentiated initially
        Specialized upon placement to assemble a single type of item
        Specialization is permanent
        Specialization choices are key to a player's strategy

Object placement
    Management-style interactions with the world
        There is no "player" like in Factorio
    Click on item in inventory view
        Storage container or personal inventory
    Click elsewhere on the map
        Objects can only be placed within a fixed distance from the nearest control center
        Ghost of item appears on the map following mouse hover
        Ghost remains after placement
        Worker bots travel from contol center to placement location and construct the object
        Travel takes time. Constrution takes single tick.

Trading
    Players are expected to trade by sending resources to each other
    Geographic variability creates comparative advantages which inspire trading relationships
    Specialization creates comparative advantages between players
    Strong player relationships can involve direct factory integrations
    Comparative advantages change through the course of the game
    Military protection/pressure is an implicily traded service

Terminal objects
    Worker bots
        Worker bots can increase efficiency of industry node
    Killer bots
    Victory points (compute/U-235)

## Competition

Subterfuge-like slow tick rate

Players are expected to attack infrastructure
    Destroyed infrastructure leaves "ghost" traces where it used to be
        Can be repaired at significantly lower cost than new construction
    Development is implicily slowed by the fragility of investment
        Defenses must be developed alongside infrastructure
        Serious risk evaluation during expansion
        Treaties with other players allow more efficient balance of military resources

On average, the player with maximal consumption should win
    Other players are expected to align against the leader to balance power
    Accolades may be given for second or third place (e.g. Subterfuge) to minimize blue-shell behavior

Victory
    Refine enough U-235 to build a nuclear weapon
    Requires siphoning industrial surplus to victory point resource generation
    Victory points are not functional
    Like anything else, trading can increase industrial capacity
        Viable win path for a more peaceful players

## Development

Stage 0
    Bootstrapping
    Each player starts with a standard set of objects
        Control center
            Home of worker bots
            Unlimited workers
        Battery
            Provides temporary power
            Can be recharged
        Materializer
            Can manufacture any item, but not instantly
    Establish initial friend-enemy relationships

Stage 1
    Vertical development of industry toward military capacity
    Set up trade deals

Stage 2
    Horizontal scaling of industry
    Limited military skirmishes for resource access
    Serious military action requires other borders to be secure
    Relationships are strained

Stage 3
    Horizontal scaling of industry
    Large gap behind leading players
    Autarky becomes viable
    Elimination becomes viable

## User interface

Map
Inventory
Assembler specializer
Time machine

Map always renders the same resolution at all screen sizes
    User with larger screen can see more of the map, but this is not an issue since any user can simply scroll to see the rest of the map

