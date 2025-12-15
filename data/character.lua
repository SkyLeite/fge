return {
    id = "mbaccakiha",
    name = "Akiha",
    max_health = 2000,
    spritesheets = {
        standing = { file = "akiha_idle.png", columns = 12, rows = 1, width = 3072, height = 256 },
        c5a = { file = "akiha_c5a.png", columns = 6, rows = 1, width = 1536, height = 256 },
        walk_forward = { file = "akiha_walk_forward.png", columns = 13, rows = 1, width = 3328, height = 256 },
        walk_backward = { file = "akiha_walk_backward.png", columns = 13, rows = 1, width = 3328, height = 256 },
    },
    states = {
        standing = {
            commands = {
                {
                    action = {
                        SetHitboxes = {
                            { x = 0, y = 0, w = 200, h = 200 }
                        }
                    },
                    frames = { from = 4, to = 45 }
                }
            },
            cancels = {
                c5a = {},
            }
        },
        walk_forward = {
            commands = {},
            cancels = {}
        },
        walk_backward = {
            commands = {},
            cancels = {}
        },
        c5a = {
            commands = {
                {
                    action = {
                        SetHitboxes = {
                            { x = 0, y = 0, w = 10, h = 10 }
                        }
                    },
                },
            },
            input = "A",
            cancels = {}
        },
    },
    animations = {
        standing = {
            Sprite = {
                repetition = "loop",
                default_collision_box = {
                    x = 0,
                    y = -35,
                    w = 30,
                    h = 100
                },
                frames = {
                    { cell = { 0, 0 },  duration = 4 },
                    { cell = { 1, 0 },  duration = 5 },
                    { cell = { 2, 0 },  duration = 6 },
                    { cell = { 3, 0 },  duration = 6 },
                    { cell = { 4, 0 },  duration = 6 },
                    { cell = { 5, 0 },  duration = 6 },
                    { cell = { 6, 0 },  duration = 6 },
                    { cell = { 7, 0 },  duration = 6 },
                    { cell = { 8, 0 },  duration = 6 },
                    { cell = { 9, 0 },  duration = 6 },
                    { cell = { 10, 0 }, duration = 6 },
                    { cell = { 11, 0 }, duration = 6 },
                }
            }
        },
        c5a = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = -35,
                    w = 30,
                    h = 100
                },
                frames = {
                    { cell = { 0, 0 }, duration = 4 },
                    { cell = { 1, 0 }, duration = 5 },
                    { cell = { 2, 0 }, duration = 6 },
                    { cell = { 3, 0 }, duration = 6 },
                    { cell = { 4, 0 }, duration = 6 },
                    { cell = { 5, 0 }, duration = 6 },
                }
            }
        },
        pre_walk_forward = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = -35,
                    w = 30,
                    h = 100
                },
                frames = {
                    { sheet = "walk_forward", cell = { 0, 0 }, duration = 6 }
                }
            }
        },
        walk_forward = {
            Sprite = {
                repetition = "loop",
                default_collision_box = {
                    x = 0,
                    y = -35,
                    w = 30,
                    h = 100
                },
                frames = {
                    { cell = { 1, 0 },  duration = 6 },
                    { cell = { 2, 0 },  duration = 6 },
                    { cell = { 3, 0 },  duration = 6 },
                    { cell = { 4, 0 },  duration = 6 },
                    { cell = { 5, 0 },  duration = 6 },
                    { cell = { 6, 0 },  duration = 6 },
                    { cell = { 7, 0 },  duration = 6 },
                    { cell = { 8, 0 },  duration = 6 },
                    { cell = { 9, 0 },  duration = 6 },
                    { cell = { 10, 0 }, duration = 6 },
                    { cell = { 11, 0 }, duration = 6 },
                    { cell = { 12, 0 }, duration = 6 },
                }
            }
        },
        walk_backward = {
            Sprite = {
                repetition = "loop",
                default_collision_box = {
                    x = 0,
                    y = -35,
                    w = 30,
                    h = 100
                },
                frames = {
                    { cell = { 1, 0 },  duration = 6 },
                    { cell = { 2, 0 },  duration = 6 },
                    { cell = { 3, 0 },  duration = 6 },
                    { cell = { 4, 0 },  duration = 6 },
                    { cell = { 5, 0 },  duration = 6 },
                    { cell = { 6, 0 },  duration = 6 },
                    { cell = { 7, 0 },  duration = 6 },
                    { cell = { 8, 0 },  duration = 6 },
                    { cell = { 9, 0 },  duration = 6 },
                    { cell = { 10, 0 }, duration = 6 },
                    { cell = { 11, 0 }, duration = 6 },
                    { cell = { 12, 0 }, duration = 6 },
                }
            }
        }
    }
}
