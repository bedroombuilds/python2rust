import xml.etree.ElementTree as ET

if __name__ == "__main__":
    tree = ET.parse('../cd-list.xml')
    root = tree.getroot()

    for child in root:
        print('childnode', child)

    for title in root.iter('title'):
        print('title', title.text)

    for track in root.iter('track'):
        print('track-no', track.attrib['no'])

    newcd = ET.SubElement(root, 'cd')
    artist = ET.SubElement(newcd, 'artist')
    artist.text = "Bob"
    title = ET.SubElement(newcd, 'title')
    title.text = "Song for Alice"
    ET.dump(root)
