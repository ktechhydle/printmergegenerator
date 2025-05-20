import streamlit as st
from print_merge_generator import generate_numbers


st.set_page_config(page_title='Print Merge Generator', page_icon='resources/icons/logos/printmergegenerator_icon.svg', layout='wide')
st.markdown('<h2 style="text-align:center">Print Merge Number Generator for Corel Draw and InDesign</h2>',
            unsafe_allow_html=True)
layout1, layout2 = st.columns(2)

with layout1:
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

missing_fields = []

if not name:
    missing_fields.append('File Name')

if not prefix:
    missing_fields.append('Prefix')

with layout2:
    if not missing_fields:
        file_content = generate_numbers(prefix, int(column_count), range_start, range_stop,
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
