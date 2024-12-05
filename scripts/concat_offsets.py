from os import system
from sys import argv

offset_file: str = argv[1]

# Sorting the file
system(f"sort -h -o {offset_file} {offset_file}")

current_node_name: str = ''
current_size: int = 0
node_occurences_per_path: dict[str, list[tuple[int, int, str]]] = {}

for line in open(offset_file, 'r'):
    if not line.startswith('#'):
        datas = line.strip().split('\t')
        if current_node_name != (node_name := datas[0]) and current_size != 0:
            # Write to stdio the node datas
            print(
                f"{current_node_name}\t{current_size}\t{chr(9).join([f'{path}:{occurences}' for path, occurences in node_occurences_per_path.items()])}")
            # Reset the structures
            current_node_name = node_name
            node_occurences_per_path = {}
        # In every other case, we add the occurence to the path
        current_node_name = datas[0]
        path = datas[1]
        start_position = int(datas[2])
        stop_position = int(datas[3])
        current_size = int(datas[4])
        orientation = datas[5]
        if path in node_occurences_per_path:
            node_occurences_per_path[path].append(
                (start_position, stop_position, orientation))
        else:
            node_occurences_per_path[path] = [
                (start_position, stop_position, orientation)]
