return {
    id = "mbaccakiha",
    name = "Akiha",
    max_health = 2000,
    spritesheets = {
        standing = { file = "akiha_idle.png", columns = 12, rows = 1, width = 3072, height = 256 },
        c5a = { file = "akiha_c5a.png", columns = 6, rows = 1, width = 672, height = 112 },
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
                pre_walk_forward = {},
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
            cancels = {
                attack_medium = {}
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
        },
        c5a = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = 0,
                    w = 30,
                    h = 100
                },
                frames = {
                    { sheet = "c5a", cell = { 0, 0 }, duration = 4 },
                    { sheet = "c5a", cell = { 1, 0 }, duration = 5 },
                    { sheet = "c5a", cell = { 2, 0 }, duration = 6 },
                    { sheet = "c5a", cell = { 3, 0 }, duration = 6 },
                    { sheet = "c5a", cell = { 4, 0 }, duration = 6 },
                    { sheet = "c5a", cell = { 5, 0 }, duration = 6 },
                }
            }
        },
        pre_walk_forward = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = 0,
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
                default_collision_box = {
                    x = 0,
                    y = 0,
                    w = 30,
                    h = 100
                },
                frames = {
                    { sheet = "walk_forward", cell = { 1, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 2, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 3, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 4, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 5, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 6, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 7, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 8, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 9, 0 },  duration = 6 },
                    { sheet = "walk_forward", cell = { 10, 0 }, duration = 6 },
                    { sheet = "walk_forward", cell = { 11, 0 }, duration = 6 },
                    { sheet = "walk_forward", cell = { 12, 0 }, duration = 6 },
                }
            }
        },
        walk_backward = {
            Sprite = {
                default_collision_box = {
                    x = 0,
                    y = 0,
                    w = 30,
                    h = 100
                },
                frames = {
                    { sheet = "walk_backward", cell = { 1, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 2, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 3, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 4, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 5, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 6, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 7, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 8, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 9, 0 },  duration = 6 },
                    { sheet = "walk_backward", cell = { 10, 0 }, duration = 6 },
                    { sheet = "walk_backward", cell = { 11, 0 }, duration = 6 },
                    { sheet = "walk_backward", cell = { 12, 0 }, duration = 6 },
                }
            }
        }
    }
}
