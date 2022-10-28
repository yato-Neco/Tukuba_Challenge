from tflite_support.metadata_writers import object_detector
from tflite_support.metadata_writers import writer_utils

ImageClassifierWriter = object_detector.MetadataWriter
_MODEL_PATH = "efficientdet-lite-salad.tflite"
_LABEL_FILE = "salad-labels.txt"
_SAVE_TO_PATH = "efficientdet-lite-salad_metadata.tflite"

_INPUT_NORM_MEAN = 127.5
_INPUT_NORM_STD = 127.5

writer = ImageClassifierWriter.create_for_inference(
    writer_utils.load_file(_MODEL_PATH), [_INPUT_NORM_MEAN], [_INPUT_NORM_STD],
    [_LABEL_FILE])
print(writer.get_metadata_json())
writer_utils.save_file(writer.populate(), _SAVE_TO_PATH)