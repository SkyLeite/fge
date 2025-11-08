local sprites = {
    { file = "psylocke1.png "},
    { file = "psylocke2.png "},
    { file = "psylocke3.png "},
    { file = "psylocke4.png "},
}

return {
    id = "sf4viper",
    name = "C. Viper",
    max_health = 2000,
    states = {
        standing = {
            commands = {
                {
                    action = { SetAnimation = "standing" }
                }
            }
        }
    },
    animations = {
        standing = {
            Sprite = {
                frames = {
                    { sprite = sprites[1], duration = 1 },
                    { sprite = sprites[2], duration = 1 },
                    { sprite = sprites[3], duration = 1 },
                }
            }
        }
    }
}
