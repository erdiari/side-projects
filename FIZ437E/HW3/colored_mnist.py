import numpy as np
from torch.utils.data import Dataset
from torch import from_numpy
from PIL import Image


class coloredMNIST(Dataset):
    def __init__(self, data_dir, test=False, transform=None, target_transform=None):
        # super(coloredMNIST, self).__init__(data_dir, test=test, transfor=transform, target_transform=target_transform)
        self.transform = transform
        self.target_transform = target_transform
        self.is_test = test
        if self.is_test:
            self.data = from_numpy(np.load(data_dir + 'numpy_test_data.npz')['arr_0'])
            self.target = from_numpy(np.load(data_dir + 'numpy_test_label.npz')['arr_0'])
        else:
            self.data = from_numpy(np.load(data_dir + 'numpy_train_data.npz')['arr_0'])
            self.target = from_numpy(np.load(data_dir + 'numpy_train_label.npz')['arr_0'])

    def __len__(self) -> int:
        return len(self.data)

    def __getitem__ (self, idx:int):
        data = self.data[idx]
        target = self.target[idx]
        data = Image.fromarray(data.numpy(), mode='RGB')
        if self.transform is not None:
            data = self.transform(data)
        if self.target_transform is not None:
            target = self.target_transform(target)
        return data, target

