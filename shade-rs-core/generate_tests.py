from typing import List, Tuple
import click
import itertools

ColorIndexed = Tuple[int, str]
ColorsCombination = List[ColorIndexed]


def all_color_combinations(colors: str = "rgba") -> List[ColorsCombination]:
    colors_indexed = [*enumerate(colors)]

    combs: List[ColorsCombination] = []
    for times in range(2, len(colors)+1):
        combs += [list(x)
                  for x in itertools.product(colors_indexed, repeat=times)]

    return combs


def generate_tests_for(colors: str, item_type: type, item_type_name: str) -> List[str]:
    vec_len = len(colors)
    vec_type_name = f"{item_type_name}{vec_len}"

    tests = []

    default_vec_value = "[" + \
        ", ".join([str(item_type(0))] * vec_len) + "]"

    for (index, color) in enumerate(colors):
        new_value = str(item_type(index))

        single_test = f"""
                #[test]
                fn test_{vec_type_name}__{color}() {{
                    let mut vector: {vec_type_name} = {default_vec_value}.into();
                    vector.set_{color}({new_value});
                    assert_eq!(vector.{color}(), {new_value});
                }}
            """

        tests.append(single_test)

    for combination in all_color_combinations(colors):
        slice_name = "".join(color for (_, color) in combination)

        indices = [index for (index, _) in combination]

        new_slice_value = "[" + ", ".join([str(item_type(index))
                                           for index in indices]) + "]"

        slice_test = f"""
                #[test]
                fn test_{vec_type_name}__{slice_name}() {{
                    let mut vector: {vec_type_name} = {default_vec_value}.into();
                    vector.set_{slice_name}({new_slice_value}.into());
                    assert_eq!(vector.{slice_name}(), {new_slice_value}.into());
                }}
            """

        tests.append(slice_test)

    return tests


@click.command()
@click.option("--type", "item_type", type=str, required=True)
@click.option("--name", "item_type_name", type=str, required=True)
@click.option("--colors", "colors", type=str, default="rgba")
def generate_tests(colors: str, item_type: str, item_type_name: str):
    vector_len = len(colors)

    tests = generate_tests_for(
        colors[:vector_len], eval(item_type), item_type_name
    )

    for test in tests:
        print(test)


if __name__ == "__main__":
    generate_tests()
