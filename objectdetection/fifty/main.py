import fiftyone as fo
import fiftyone.zoo as foz


#validation

classes = ["Cat", "Dog", "Car", "Person" ]


dataset = fo.zoo.load_zoo_dataset(
              "open-images-v6",
              dataset_dir="D:/datasets/",
              split="train",
              overwrite=True,
              label_types=["detections", "segmentations"],
              classes=classes,
              max_samples=250,
          )

session = fo.launch_app(dataset,desktop=True)
session.wait()
print("end")

dataset.export(
    export_dir="D:/efficientdet_dataset/train/",
    dataset_type=fo.types.VOCDetectionDataset,
    label_field="detections",
    overwrite=True,
    classes=classes[0],
)

