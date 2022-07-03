import xml.etree.ElementTree as ET 
import glob

print(len(glob.glob("*.xml")))


for f in glob.glob("*.xml"):
    print(f)

    tree = ET.parse('./' + f) 


    root = tree.getroot()

    for sport in root.findall('object'):
        s_population = ET.SubElement(sport, 'difficult') 
        s_population.text = "0"
        s_population = ET.SubElement(sport, 'truncated') 
        s_population.text = "0"
        s_population = ET.SubElement(sport, 'pose') 
        s_population.text = "Unspecified"

    tree.write('./result/'+ f,)

