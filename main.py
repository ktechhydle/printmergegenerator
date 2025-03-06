import streamlit as st
import csv
from io import StringIO


# Function to generate the numbered file
def generate_numbered_file(name: str, column_count: int, num_range: range, prefix, count_numbers_vertically,
                           aligned=False):
    # Adjust the range to include the stop value
    num_range = range(num_range.start, num_range.stop + 1)

    # Determine the maximum length based on the largest number in the range
    max_length = len(str(num_range[-1]))

    # Use StringIO to capture file output in memory
    output = StringIO()
    writer = csv.writer(output, delimiter=',', quoting=csv.QUOTE_MINIMAL)

    # Write the column headers dynamically based on column_count
    headers = [f'{prefix}{i + 1}' for i in range(column_count)]
    writer.writerow(headers)

    if count_numbers_vertically:
        # Calculate the number of rows needed for vertical arrangement
        rows = [[] for _ in range((len(num_range) + column_count - 1) // column_count)]

        # Divide the numbers into vertical columns
        for idx, number in enumerate(num_range):
            # Pad the number with leading zeros if aligned is True
            formatted_number = str(number).zfill(max_length) if aligned else str(number)
            # Place the number in the appropriate row and column
            row_index = idx % len(rows)
            rows[row_index].append(formatted_number)

        # Fill any short rows to match the column count
        for row in rows:
            while len(row) < column_count:
                row.append('')  # Fill with empty strings for alignment

        # Write rows to CSV
        writer.writerows(rows)

    else:
        row = []
        for i in num_range:
            # Pad the number with leading zeros if aligned is True
            number = str(i).zfill(max_length) if aligned else str(i)
            row.append(number)

            # Once row reaches column_count, write it and reset row
            if len(row) == column_count:
                writer.writerow(row)
                row = []

        # Write any remaining numbers in the last row if row is not empty
        if row:
            writer.writerow(row)

    # Move to start of the StringIO object to read contents
    output.seek(0)
    return output.getvalue()


st.set_page_config(page_title='Print Merge Generator', page_icon='icons/printmergegenerator_icon.png', layout='wide')
st.markdown('<h2 style="text-align:center">Print Merge Number Generator for Corel Draw and InDesign</h2>',
            unsafe_allow_html=True)
layout1, layout2 = st.columns(2)

with layout1:
    # Input fields
    name = st.text_input('File Name (With Extension)', 'output.txt', help='Enter the output filename along with the '
                                                                          'file extension')
    range_start = st.number_input('Start Number', min_value=0, value=1, help='Enter the starting value')
    range_stop = st.number_input('End Number', min_value=1, value=100, help='Enter the ending value')
    column_count = st.number_input('Repeating Amount', min_value=1, value=1, help='Enter the repeating amount of '
                                                                                  'columns')
    prefix = st.text_input('Prefix', 'No.', help='Enter the prefix of the column header row')

    count_num_vertically = False
    if column_count > 1:
        count_num_vertically = st.checkbox('Count Numbers Vertically', help='If this is checked, rows will '
                                                                            'count vertically')
    aligned = st.checkbox('Number Spots', help='If this is checked, each column will align with leading zeros '
                                               '(e.g. "01, 02, 03 ... 10")')

# Identify missing fields
missing_fields = []
if not name:
    missing_fields.append('File Name')
if not prefix:
    missing_fields.append('Prefix')

with layout2:
    if not missing_fields:
        file_content = generate_numbered_file(name, int(column_count), range(range_start, range_stop), prefix,
                                              count_num_vertically, aligned)
        st.text_area('Preview', file_content, height=375, help='A preview of the generated file (any changes made to '
                                                               'it will not affect the output file)')

        if st.download_button(
                label='Download File',
                data=file_content,
                file_name=name,
                mime='text/csv'
        ):
            print('downloaded')
    else:
        missing_fields_str = ', '.join(missing_fields)
        st.warning(f'Please fill in the following fields: {missing_fields_str}', icon='⚠️')

st.markdown('_Copyright (©) Keller Hydle_')
