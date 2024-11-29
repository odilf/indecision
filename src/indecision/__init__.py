# TODO: Could do some cli stuff here...
# TODO: idk if it should be `main.py` or `__main__.py` or what...

if __name__ == "__main__":
    from particle import MultiLigandParticle

    print(
        MultiLigandParticle(
            receptor_density=0.1,
            rates=[
                (0.1, 0.4),
                (0.4, 0.5),
                (0.5, 0.9),
            ],
        )
    )
