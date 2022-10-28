from re import L
from regex import R
import cv2
import numpy as np
import json




def image_size(img):
  return (len(img[0][0]),len(img[0]))


def object_cut(result, depth_img, im_width, im_height, label_id_offset, category_index,savae):


  result_list = []
  tmp_dic = {}

  for i in range(0,len(result['detection_boxes'][0][0])):


    (left, right, top, bottom) = (result['detection_boxes'][0][i][1] * im_width, result['detection_boxes'][0][i][3] * im_width,
                              result['detection_boxes'][0][i][0] * im_height, result['detection_boxes'][0][i][2] * im_height)
    #print(left,right,top,bottom)

    s_roi = depth_img[int(top): int(bottom), int(left): int(right)]

    #print((s_roi) / 255.0)

    s_roi = cv2.cvtColor(s_roi, cv2.COLOR_RGB2BGR)

    #print(type(s_roi))

    if savae:

      cv2.imwrite("./result/object/"+ str(i) +".png", s_roi)


    index = (result['detection_classes'][0] + label_id_offset).astype(int)[i]

    json_data = category_index[index]

    json_data["detection_scores"] = float(result['detection_scores'][0][i])

    json_data["mean"] = float(np.mean(s_roi / 255.0))

    result_list.append(json_data.copy())


    """with open('./result/object/' + str(i) + ".json", 'w') as f:

      json.dump(json_data, f, ensure_ascii=False)"""
      #f.write( str() + ": "  +  str()  + ": " +  str(np.mean(s_roi / 255.0)))

    #json_data.clear()


  
  return result_list





def image_save(img,path:str):
  im_bgr = cv2.cvtColor(img, cv2.COLOR_RGB2BGR)
  cv2.imwrite(path, im_bgr)



def start(file_pass,savae=False):


  import os
  import pathlib
  import matplotlib
  matplotlib.use('TkAgg')
  import matplotlib.pyplot as plt

  import io
  import scipy.misc
  from six import BytesIO
  from PIL import Image, ImageDraw, ImageFont
  from six.moves.urllib.request import urlopen

  import tensorflow as tf
  import tensorflow_hub as hub


  #depth_img = depth.return_img(file_pass,savae)

  tf.get_logger().setLevel('ERROR')


  # @title Run this!!

  def load_image_into_numpy_array(path):
    """Load an image from file into a numpy array.

    Puts image into numpy array to feed into tensorflow graph.
    Note that by convention we put it into a numpy array with shape
    (height, width, channels), where channels=3 for RGB.

    Args:
      path: the file path to the image

    Returns:
      uint8 numpy array with shape (img_height, img_width, 3)
    """
    image = None
    if(path.startswith('http')):
      response = urlopen(path)
      image_data = response.read()
      image_data = BytesIO(image_data)
      image = Image.open(image_data)
    else:
      image_data = tf.io.gfile.GFile(path, 'rb').read()
      image = Image.open(BytesIO(image_data))

    (im_width, im_height) = image.size
    return np.array(image.getdata()).reshape(
        (1, im_height, im_width, 3)).astype(np.uint8)


  ALL_MODELS = {
  'CenterNet HourGlass104 512x512' : 'https://tfhub.dev/tensorflow/centernet/hourglass_512x512/1',
  'CenterNet HourGlass104 Keypoints 512x512' : 'https://tfhub.dev/tensorflow/centernet/hourglass_512x512_kpts/1',
  'CenterNet HourGlass104 1024x1024' : 'https://tfhub.dev/tensorflow/centernet/hourglass_1024x1024/1',
  'CenterNet HourGlass104 Keypoints 1024x1024' : 'https://tfhub.dev/tensorflow/centernet/hourglass_1024x1024_kpts/1',
  'CenterNet Resnet50 V1 FPN 512x512' : 'https://tfhub.dev/tensorflow/centernet/resnet50v1_fpn_512x512/1',
  'CenterNet Resnet50 V1 FPN Keypoints 512x512' : 'https://tfhub.dev/tensorflow/centernet/resnet50v1_fpn_512x512_kpts/1',
  'CenterNet Resnet101 V1 FPN 512x512' : 'https://tfhub.dev/tensorflow/centernet/resnet101v1_fpn_512x512/1',
  'CenterNet Resnet50 V2 512x512' : 'https://tfhub.dev/tensorflow/centernet/resnet50v2_512x512/1',
  'CenterNet Resnet50 V2 Keypoints 512x512' : 'https://tfhub.dev/tensorflow/centernet/resnet50v2_512x512_kpts/1',
  'EfficientDet D0 512x512' : 'https://tfhub.dev/tensorflow/efficientdet/d0/1',
  'EfficientDet D1 640x640' : 'https://tfhub.dev/tensorflow/efficientdet/d1/1',
  'EfficientDet D2 768x768' : 'https://tfhub.dev/tensorflow/efficientdet/d2/1',
  'EfficientDet D3 896x896' : 'https://tfhub.dev/tensorflow/efficientdet/d3/1',
  'EfficientDet D4 1024x1024' : 'https://tfhub.dev/tensorflow/efficientdet/d4/1',
  'EfficientDet D5 1280x1280' : 'https://tfhub.dev/tensorflow/efficientdet/d5/1',
  'EfficientDet D6 1280x1280' : 'https://tfhub.dev/tensorflow/efficientdet/d6/1',
  'EfficientDet D7 1536x1536' : 'https://tfhub.dev/tensorflow/efficientdet/d7/1',
  'SSD MobileNet v2 320x320' : 'https://tfhub.dev/tensorflow/ssd_mobilenet_v2/2',
  'SSD MobileNet V1 FPN 640x640' : 'https://tfhub.dev/tensorflow/ssd_mobilenet_v1/fpn_640x640/1',
  'SSD MobileNet V2 FPNLite 320x320' : 'https://tfhub.dev/tensorflow/ssd_mobilenet_v2/fpnlite_320x320/1',
  'SSD MobileNet V2 FPNLite 640x640' : 'https://tfhub.dev/tensorflow/ssd_mobilenet_v2/fpnlite_640x640/1',
  'SSD ResNet50 V1 FPN 640x640 (RetinaNet50)' : 'https://tfhub.dev/tensorflow/retinanet/resnet50_v1_fpn_640x640/1',
  'SSD ResNet50 V1 FPN 1024x1024 (RetinaNet50)' : 'https://tfhub.dev/tensorflow/retinanet/resnet50_v1_fpn_1024x1024/1',
  'SSD ResNet101 V1 FPN 640x640 (RetinaNet101)' : 'https://tfhub.dev/tensorflow/retinanet/resnet101_v1_fpn_640x640/1',
  'SSD ResNet101 V1 FPN 1024x1024 (RetinaNet101)' : 'https://tfhub.dev/tensorflow/retinanet/resnet101_v1_fpn_1024x1024/1',
  'SSD ResNet152 V1 FPN 640x640 (RetinaNet152)' : 'https://tfhub.dev/tensorflow/retinanet/resnet152_v1_fpn_640x640/1',
  'SSD ResNet152 V1 FPN 1024x1024 (RetinaNet152)' : 'https://tfhub.dev/tensorflow/retinanet/resnet152_v1_fpn_1024x1024/1',
  'Faster R-CNN ResNet50 V1 640x640' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet50_v1_640x640/1',
  'Faster R-CNN ResNet50 V1 1024x1024' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet50_v1_1024x1024/1',
  'Faster R-CNN ResNet50 V1 800x1333' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet50_v1_800x1333/1',
  'Faster R-CNN ResNet101 V1 640x640' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet101_v1_640x640/1',
  'Faster R-CNN ResNet101 V1 1024x1024' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet101_v1_1024x1024/1',
  'Faster R-CNN ResNet101 V1 800x1333' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet101_v1_800x1333/1',
  'Faster R-CNN ResNet152 V1 640x640' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet152_v1_640x640/1',
  'Faster R-CNN ResNet152 V1 1024x1024' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet152_v1_1024x1024/1',
  'Faster R-CNN ResNet152 V1 800x1333' : 'https://tfhub.dev/tensorflow/faster_rcnn/resnet152_v1_800x1333/1',
  'Faster R-CNN Inception ResNet V2 640x640' : 'https://tfhub.dev/tensorflow/faster_rcnn/inception_resnet_v2_640x640/1',
  'Faster R-CNN Inception ResNet V2 1024x1024' : 'https://tfhub.dev/tensorflow/faster_rcnn/inception_resnet_v2_1024x1024/1',
  'Mask R-CNN Inception ResNet V2 1024x1024' : 'https://tfhub.dev/tensorflow/mask_rcnn/inception_resnet_v2_1024x1024/1'
  }

  IMAGES_FOR_TEST = {
    'Beach' : 'models/research/object_detection/test_images/image2.jpg',
    'Dogs' : 'models/research/object_detection/test_images/image1.jpg',
    # By Heiko Gorski, Source: https://commons.wikimedia.org/wiki/File:Naxos_Taverna.jpg
    'Naxos Taverna' : 'https://upload.wikimedia.org/wikipedia/commons/6/60/Naxos_Taverna.jpg',
    # Source: https://commons.wikimedia.org/wiki/File:The_Coleoptera_of_the_British_islands_(Plate_125)_(8592917784).jpg
    'Beatles' : 'https://upload.wikimedia.org/wikipedia/commons/1/1b/The_Coleoptera_of_the_British_islands_%28Plate_125%29_%288592917784%29.jpg',
    # By Américo Toledano, Source: https://commons.wikimedia.org/wiki/File:Biblioteca_Maim%C3%B3nides,_Campus_Universitario_de_Rabanales_007.jpg
    'Phones' : 'https://upload.wikimedia.org/wikipedia/commons/thumb/0/0d/Biblioteca_Maim%C3%B3nides%2C_Campus_Universitario_de_Rabanales_007.jpg/1024px-Biblioteca_Maim%C3%B3nides%2C_Campus_Universitario_de_Rabanales_007.jpg',
    # Source: https://commons.wikimedia.org/wiki/File:The_smaller_British_birds_(8053836633).jpg
    'Birds' : 'https://upload.wikimedia.org/wikipedia/commons/0/09/The_smaller_British_birds_%288053836633%29.jpg',
  }

  COCO17_HUMAN_POSE_KEYPOINTS = [(0, 1),
  (0, 2),
  (1, 3),
  (2, 4),
  (0, 5),
  (0, 6),
  (5, 7),
  (7, 9),
  (6, 8),
  (8, 10),
  (5, 6),
  (5, 11),
  (6, 12),
  (11, 12),
  (11, 13),
  (13, 15),
  (12, 14),
  (14, 16)]

  from object_detection.utils import label_map_util
  from object_detection.utils import visualization_utils as viz_utils
  from object_detection.utils import ops as utils_ops

  PATH_TO_LABELS = './models/research/object_detection/data/mscoco_label_map.pbtxt'
  category_index = label_map_util.create_category_index_from_labelmap(PATH_TO_LABELS, use_display_name=True)

  model_display_name = 'SSD MobileNet v2 320x320' # @param ['CenterNet HourGlass104 512x512','CenterNet HourGlass104 Keypoints 512x512','CenterNet HourGlass104 1024x1024','CenterNet HourGlass104 Keypoints 1024x1024','CenterNet Resnet50 V1 FPN 512x512','CenterNet Resnet50 V1 FPN Keypoints 512x512','CenterNet Resnet101 V1 FPN 512x512','CenterNet Resnet50 V2 512x512','CenterNet Resnet50 V2 Keypoints 512x512','EfficientDet D0 512x512','EfficientDet D1 640x640','EfficientDet D2 768x768','EfficientDet D3 896x896','EfficientDet D4 1024x1024','EfficientDet D5 1280x1280','EfficientDet D6 1280x1280','EfficientDet D7 1536x1536','SSD MobileNet v2 320x320','SSD MobileNet V1 FPN 640x640','SSD MobileNet V2 FPNLite 320x320','SSD MobileNet V2 FPNLite 640x640','SSD ResNet50 V1 FPN 640x640 (RetinaNet50)','SSD ResNet50 V1 FPN 1024x1024 (RetinaNet50)','SSD ResNet101 V1 FPN 640x640 (RetinaNet101)','SSD ResNet101 V1 FPN 1024x1024 (RetinaNet101)','SSD ResNet152 V1 FPN 640x640 (RetinaNet152)','SSD ResNet152 V1 FPN 1024x1024 (RetinaNet152)','Faster R-CNN ResNet50 V1 640x640','Faster R-CNN ResNet50 V1 1024x1024','Faster R-CNN ResNet50 V1 800x1333','Faster R-CNN ResNet101 V1 640x640','Faster R-CNN ResNet101 V1 1024x1024','Faster R-CNN ResNet101 V1 800x1333','Faster R-CNN ResNet152 V1 640x640','Faster R-CNN ResNet152 V1 1024x1024','Faster R-CNN ResNet152 V1 800x1333','Faster R-CNN Inception ResNet V2 640x640','Faster R-CNN Inception ResNet V2 1024x1024','Mask R-CNN Inception ResNet V2 1024x1024']
  model_handle = ALL_MODELS[model_display_name]

  print('Selected model:'+ model_display_name)
  print('Model Handle at TensorFlow Hub: {}'.format(model_handle))


  print('loading model...')
  hub_model = hub.load(model_handle)
  print('model loaded!')

  flip_image_horizontally = False
  convert_image_to_grayscale = False

  image_path = file_pass #IMAGES_FOR_TEST[selected_image]
  image_np = load_image_into_numpy_array(image_path)

  # Flip horizontally
  if(flip_image_horizontally):
    image_np[0] = np.fliplr(image_np[0]).copy()

  # Convert image to grayscale
  if(convert_image_to_grayscale):
    image_np[0] = np.tile(
      np.mean(image_np[0], 2, keepdims=True), (1, 1, 3)).astype(np.uint8)

  #plt.imshow(image_np[0])
  #plt.savefig('figure01.jpg')


  # running inference
  results = hub_model(image_np)

  # different object detection models have additional results
  # all of them are explained in the documentation
  result = {key:value.numpy() for key,value in results.items()}
  #print(result.keys())

  label_id_offset = 0
  image_np_with_detections = image_np.copy()


  #depth_img = np.array(Image.open('./result/depth.jpg').convert('RGB')).astype(np.uint8)

  (im_width, im_height) = image_size(image_np_with_detections)


  #result_list = object_cut(result, depth_img, im_width, im_height, label_id_offset, category_index,savae)




  # Use keypoints if available in detections
  keypoints, keypoint_scores = None, None
  if 'detection_keypoints' in result:
    keypoints = result['detection_keypoints'][0]
    keypoint_scores = result['detection_keypoint_scores'][0]



 

  viz_utils.visualize_boxes_and_labels_on_image_array(
        image_np_with_detections[0],
        result['detection_boxes'][0],
        (result['detection_classes'][0] + label_id_offset).astype(int),
        result['detection_scores'][0],
        category_index,
        use_normalized_coordinates=True,
        max_boxes_to_draw=200,
        min_score_thresh=.30,
        agnostic_mode=False,
        keypoints=keypoints,
        keypoint_scores=keypoint_scores,
        keypoint_edges=COCO17_HUMAN_POSE_KEYPOINTS)

  

  if savae:
    image_save(
      img=image_np_with_detections[0],
      path="./result/object.png"
    )

  
  
  
  #result_list.reverse()


  #return result_list



