import streamlit as st
import csv
from io import StringIO


# Function to generate the numbered file
def generate_numbered_file(name: str, column_count: int, num_range: range, prefix, aligned=False):
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

    # Prepare and write each row
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


# Streamlit interface
st.set_page_config(page_title='Print Merge Generator', page_icon='icons/print_merge_gen_logo.png')
st.markdown('### Print Merge Number Generator for Corel Draw and InDesign\n'
            '_Copyright (©) Keller Hydle_')

# Input fields
name = st.text_input('File Name (With Extension)', 'output.txt')
range_start = st.number_input('Start Number', min_value=1, value=1)
range_stop = st.number_input('End Number', min_value=1, value=100)
column_count = st.number_input('Repeating Amount', min_value=1, value=1)
prefix = st.text_input('Prefix', 'No.')
aligned = st.checkbox('Number Spots')

# Generate file when the button is clicked
if st.button('Generate File'):
    # Generate file content
    file_content = generate_numbered_file(name, int(column_count), range(range_start, range_stop), prefix,
                                          aligned)

    # Provide download button for the generated file
    st.download_button(
        label='Download File',
        data=file_content,
        file_name=name,
        mime='text/csv'
    )
