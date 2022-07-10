
import argparse
import time

from PIL import Image
from PIL import ImageDraw

import detect
import tflite_runtime.interpreter as tflite
import platform



class CallTFlite(object):

    t = None

    def __init__(self):
        self.t = "hello"
    

    def load_labels(self, path, encoding='utf-8'):
        """Loads labels from file (with or without index numbers).

        Args:
            path: path to label file.
            encoding: label file encoding.
        Returns:
            Dictionary mapping indices to labels.
        """
        with open(path, 'r', encoding=encoding) as f:
            lines = f.readlines()
            if not lines:
                return {}

            if lines[0].split(' ', maxsplit=1)[0].isdigit():
                pairs = [line.split(' ', maxsplit=1) for line in lines]
                return {int(index): label.strip() for index, label in pairs}
            else:
                return {index: line.strip() for index, line in enumerate(lines)}
    

    def make_interpreter(self,model_file):
        model_file, *device = model_file.split('@')
        """return tflite.Interpreter(
            model_path=model_file,
            experimental_delegates=[
            tflite.load_delegate(EDGETPU_SHARED_LIB,
                               {'device': device[0]} if device else {})
        ])"""

        return tflite.Interpreter(model_path=model_file)


    def hello(self):
        return self.t

    def start(self):
        labels = self.load_labels("./scripts/coco_labels.txt") if "./scripts/coco_labels.txt" else {}
        interpreter =self. make_interpreter("./scripts/lite-model_efficientdet_lite0_int8_1.tflite")
        interpreter.allocate_tensors()

        image = Image.open("./scripts/dog2.jpg")
        scale = detect.set_input(interpreter, image.size,
                           lambda size: image.resize(size, Image.ANTIALIAS))

        interpreter.invoke()
        objs = detect.get_output(interpreter, 0.4, scale)

        if not objs:
            print('No objects detected')

        for obj in objs:
            print(labels.get(obj.id, obj.id))
            print('  id:    ', obj.id)
            print('  score: ', obj.score)
            print('  bbox:  ', obj.bbox)

