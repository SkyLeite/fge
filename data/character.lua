return {
    id = "mbaccakiha",
    name = "Akiha",
    max_health = 2000,
    spritesheets = {
        idle = { file = "akiha_idle.png", columns = 12, rows = 1, width = 768, height = 112 },
    },
    states = {
        idle = {
            commands = {
                {
                    action = { SetAnimation = "idle" }
                }
            }
        }
    },
    animations = {
        idle = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = 0,
                    w = 10,
                    h = 10
                },
                frames = {
                    { sheet = "idle", cell = { 0, 0 },  duration = 4 },
                    { sheet = "idle", cell = { 1, 0 },  duration = 5 },
                    { sheet = "idle", cell = { 2, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 3, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 4, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 5, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 6, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 7, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 8, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 9, 0 },  duration = 6 },
                    { sheet = "idle", cell = { 10, 0 }, duration = 6 },
                    { sheet = "idle", cell = { 11, 0 }, duration = 6 },
                }
            }
        }
    }
}
