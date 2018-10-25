import re

WARN_LOC_RE = re.compile('--> (.*):([0-9]*):([0-9]*)')
IMPORT_ITEM_RE = re.compile('warning: unused imports?: (`.*`)*')

# TODO: Actually run `cargo check`, rather than grabbing from a file.
def main():
    unused_import_blocks = parse_unused_import_blocks()
    # Group warn blocks by the file they occur in.  Otherwise, we'd have to rerun
    # the compiler between deletions, because the line numbers would have changed.
    file_name_to_block_list = group_blocks_by_file(unused_import_blocks)
    updated_sources = remove_unused_imports(file_name_to_block_list)
    write_updates(updated_sources)

def parse_unused_import_blocks():
    with open('compile_log.txt', 'r') as f:
        line_iter = iter(map(lambda s: s.strip(), f.readlines()))
        unused_import_blocks = []
        for line in line_iter:
            if line.startswith('warning: unused import'):
                block = []
                while line != '':
                    block.append(line)
                    line = next(line_iter)
                unused_import_blocks.append(block)
    return unused_import_blocks

def group_blocks_by_file(unused_import_blocks):
    file_name_to_block_list = {}
    for block in unused_import_blocks:
        warn_file = WARN_LOC_RE.match(block[1]).group(1)
        if warn_file not in file_name_to_block_list:
            file_name_to_block_list[warn_file] = []
        file_name_to_block_list[warn_file].append(block)
    return file_name_to_block_list

def remove_unused_imports(file_name_to_block_list):
    updated_sources = {}
    for file_name in file_name_to_block_list:
        with open(file_name, 'r') as f:
            src_lines = list(map(lambda s: s.rstrip(), f.readlines()))
        to_delete = []
        for block in file_name_to_block_list[file_name]:
            item_names = IMPORT_ITEM_RE.match(block[0]).group(1)
            item_names = list(map(lambda s: s[1:-1], item_names.split(', ')))
            warn_file, warn_row, warn_col = WARN_LOC_RE.match(block[1]).group(1, 2, 3)
            # IT DOESN'T USE 0-BASED INDEXING.  https://youtu.be/kcxal9VAFww?t=23
            warn_row = int(warn_row) - 1
            warn_col = int(warn_col) - 1
            src_line = src_lines[warn_row]
            new_src_line = remove_item_from_import(src_line, item_names)
            if new_src_line is None:
                to_delete.append(warn_row)
            else:
                src_lines[warn_row] = new_src_line

        # Remove dups and process deletions in reverse, so indices remain valid.
        print('deleting lines in {}...'.format(file_name))
        to_delete = sorted(list(set(to_delete)), reverse=True)
        for line_num in to_delete:
            del src_lines[line_num]

        # Format and store the updated source.
        updated_src = '\n'.join(src_lines)
        # We want a trailing newline for our files (it's the UNIX way!).
        updated_src += '\n'
        updated_sources[file_name] = updated_src
    return updated_sources

def write_updates(updated_sources):
    for file_name, updated_src in updated_sources.items():
        # Write to file (with unused imports now deleted).
        with open(file_name, 'w') as f:
            f.write(updated_src)

def remove_item_from_import(src_line, target_items):
    """Returns the multi import without `target_items`.

    Returns `None` if `src_line` should be deleted (i.e., nothing being
    imported after removals).
    """
    if '{' not in src_line:  # Single  import
        return None
    prefix, rest = src_line.split('{')
    items, suffix = rest.split('}')
    items = list(map(lambda s: s.strip(),
            src_line[src_line.index('{')+1:src_line.index('}')].split(',')))
    # Remove all of the target items.
    for target_item in target_items:
        items.remove(target_item)
    # Sort the imports to curb my OCD.
    items.sort()
    if len(items) == 0:
        return None
    if len(items) == 1:  # Don't need brackets.
        return '%s%s%s' % (prefix, ', '.join(items), suffix)
    else:  # Do need 'em.
        return '%s{%s}%s' % (prefix, ', '.join(items), suffix)

# Tests for `remove_from_import`
#print(remove_from_import('    use serde::{Deserialize, Serialize};',
#    'Serialize'))
#print(remove_from_import('    use serde::{Serialize};',
#    'Serialize'))
#print(remove_from_import('    use serde::{Deserialize, Serialize, Biscuits};',
#    'Deserialize', 'Biscuits'))
#print(remove_from_import('    use serde::{Deserialize, Serialize};',
#    'Deserialize', 'Serialize'))

if __name__ == '__main__':
    main()
