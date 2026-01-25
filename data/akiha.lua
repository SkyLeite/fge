return {
    id = "mbaccakiha",
    name = "Akiha",
    max_health = 2000,
    spritesheets = {
        standing = { file = "akiha_idle.png", columns = 12, rows = 1, width = 3072, height = 256 },
        c5a = { file = "akiha_c5a.png", columns = 6, rows = 1, width = 1536, height = 256 },
        c5b1 = { file = "akiha_c5b1.png", columns = 10, rows = 1, width = 2560, height = 256 },
        walk_forward = { file = "akiha_walk_forward.png", columns = 13, rows = 1, width = 3328, height = 256 },
        walk_backward = { file = "akiha_walk_backward.png", columns = 13, rows = 1, width = 3328, height = 256 },
    },
    states = {
        grounded_neutral = {
            commands = {},
            cancels = {},
            on = {
                enter = {
                    transitions = {
                        { to = "walk_forward",  when = { kind = "hold", inputs = "6" } },
                        { to = "walk_backward", when = { kind = "hold", inputs = "4" } },
                    }
                }
            }
        },
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
                walk_forward = {},
                walk_backward = {},
                c5a = {},
                c5b1 = {},
            }
        },
        walk_forward = {
            commands = {},
            cancels = {
                c5a = {},
                c5b1 = {},
                walk_backward = {},
            },
            -- on = {
            --     ["exit"] = {
            --         transitions = {
            --             { to = "grounded_neutral" }
            --         }
            --     }
            -- }
        },
        walk_backward = {
            commands = {},
            cancels = {
                c5a = {},
                c5b1 = {},
                walk_forward = {},
            },
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
            input = {
                step_sets = {
                    {
                        steps = {
                            { kind = "press", inputs = "A" },
                        }
                    },
                }
            },
            cancels = {}
        },
        c5b1 = {
            commands = {
                {
                    action = {
                        SetHitboxes = {
                            { x = 0, y = 0, w = 10, h = 10 }
                        }
                    },
                },
            },
            input = {
                step_sets = {
                    {
                        steps = {
                            { kind = "press", inputs = "B" },
                        }
                    },
                }
            },
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
        c5b1 = {
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
