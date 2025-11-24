return {
    id = "mbaccakiha",
    name = "Akiha",
    max_health = 2000,
    spritesheets = {
        standing = { file = "akiha_idle.png", columns = 12, rows = 1, width = 768, height = 112 },
    },
    states = {
        standing = {
            commands = {
                {
                    action = { SetAnimation = "standing" }
                },
                {
                    action = {
                        SetHitboxes = {
                            { x = 0, y = 0, w = 200, h = 200 }
                        }
                    },
                    condition = "frame >= 4 && frame <= 8",
                },
            }
        },
        attack_light = {
            commands = {
                {
                    action = {
                        SetHitboxes = {
                            { x = 0, y = 0, w = 10, h = 10 }
                        }
                    },
                    condition = "frame >= 4 && frame <= 8",
                },
            }
        },
    },
    animations = {
        standing = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = 0,
                    w = 30,
                    h = 100
                },
                frames = {
                    { sheet = "standing", cell = { 0, 0 },  duration = 4 },
                    { sheet = "standing", cell = { 1, 0 },  duration = 5 },
                    { sheet = "standing", cell = { 2, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 3, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 4, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 5, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 6, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 7, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 8, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 9, 0 },  duration = 6 },
                    { sheet = "standing", cell = { 10, 0 }, duration = 6 },
                    { sheet = "standing", cell = { 11, 0 }, duration = 6 },
                }
            }
        }
    }
}
