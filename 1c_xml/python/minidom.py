import xml.dom.minidom as md
import io

if __name__ == "__main__":
    dom = md.parse('../cd-list.xml')
    for child in dom.childNodes[0].childNodes:
        print('childnode', child)

    for title in dom.getElementsByTagName('title'):
        print('title', title.firstChild.nodeValue)

    for track in dom.getElementsByTagName('track'):
        print('track-no', track.getAttribute('no'))

    root = dom.childNodes[0]
    newcd = dom.createElement('cd')
    artist = dom.createElement('artist')
    artist.appendChild(dom.createTextNode('Bob'))
    artist.setAttribute('country', 'DE')
    newcd.appendChild(artist)
    title = dom.createElement('title')
    title.appendChild(dom.createTextNode('Song for Alice'))
    newcd.appendChild(title)
    root.appendChild(newcd)
    fd = io.StringIO()
    dom.writexml(fd)
    print(fd.getvalue())
