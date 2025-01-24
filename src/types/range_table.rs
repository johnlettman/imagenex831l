#![allow(clippy::approx_constant)]

pub type RangeTable = [f32; 250];

// Range: 0.125m
pub(crate) const X0_125M: RangeTable = [
    0.0005, 0.001, 0.0015, 0.002, 0.0025, 0.003, 0.0035, 0.004, 0.0045, 0.005, 0.0055, 0.006,
    0.0065, 0.007, 0.0075, 0.008, 0.0085, 0.009, 0.0095, 0.01, 0.0105, 0.011, 0.0115, 0.012,
    0.0125, 0.013, 0.0135, 0.014, 0.0145, 0.015, 0.0155, 0.016, 0.0165, 0.017, 0.0175, 0.018,
    0.0185, 0.019, 0.0195, 0.02, 0.0205, 0.021, 0.0215, 0.022, 0.0225, 0.023, 0.0235, 0.024,
    0.0245, 0.025, 0.0255, 0.026, 0.0265, 0.027, 0.0275, 0.028, 0.0285, 0.029, 0.0295, 0.03,
    0.0305, 0.031, 0.0315, 0.032, 0.0325, 0.033, 0.0335, 0.034, 0.0345, 0.035, 0.0355, 0.036,
    0.0365, 0.037, 0.0375, 0.038, 0.0385, 0.039, 0.0395, 0.04, 0.0405, 0.041, 0.0415, 0.042,
    0.0425, 0.043, 0.0435, 0.044, 0.0445, 0.045, 0.0455, 0.046, 0.0465, 0.047, 0.0475, 0.048,
    0.0485, 0.049, 0.0495, 0.05, 0.0505, 0.051, 0.0515, 0.052, 0.0525, 0.053, 0.0535, 0.054,
    0.0545, 0.055, 0.0555, 0.056, 0.0565, 0.057, 0.0575, 0.058, 0.0585, 0.059, 0.0595, 0.06,
    0.0605, 0.061, 0.0615, 0.062, 0.0625, 0.063, 0.0635, 0.064, 0.0645, 0.065, 0.0655, 0.066,
    0.0665, 0.067, 0.0675, 0.068, 0.0685, 0.069, 0.0695, 0.07, 0.0705, 0.071, 0.0715, 0.072,
    0.0725, 0.073, 0.0735, 0.074, 0.0745, 0.075, 0.0755, 0.076, 0.0765, 0.077, 0.0775, 0.078,
    0.0785, 0.079, 0.0795, 0.08, 0.0805, 0.081, 0.0815, 0.082, 0.0825, 0.083, 0.0835, 0.084,
    0.0845, 0.085, 0.0855, 0.086, 0.0865, 0.087, 0.0875, 0.088, 0.0885, 0.089, 0.0895, 0.09,
    0.0905, 0.091, 0.0915, 0.092, 0.0925, 0.093, 0.0935, 0.094, 0.0945, 0.095, 0.0955, 0.096,
    0.0965, 0.097, 0.0975, 0.098, 0.0985, 0.099, 0.0995, 0.1, 0.1005, 0.101, 0.1015, 0.102, 0.1025,
    0.103, 0.1035, 0.104, 0.1045, 0.105, 0.1055, 0.106, 0.1065, 0.107, 0.1075, 0.108, 0.1085,
    0.109, 0.1095, 0.11, 0.1105, 0.111, 0.1115, 0.112, 0.1125, 0.113, 0.1135, 0.114, 0.1145, 0.115,
    0.1155, 0.116, 0.1165, 0.117, 0.1175, 0.118, 0.1185, 0.119, 0.1195, 0.12, 0.1205, 0.121,
    0.1215, 0.122, 0.1225, 0.123, 0.1235, 0.124, 0.1245, 0.125,
];

// Range: 0.25m
pub(crate) const X0_25M: RangeTable = [
    0.001, 0.002, 0.003, 0.004, 0.005, 0.006, 0.007, 0.008, 0.009, 0.01, 0.011, 0.012, 0.013,
    0.014, 0.015, 0.016, 0.017, 0.018, 0.019, 0.02, 0.021, 0.022, 0.023, 0.024, 0.025, 0.026,
    0.027, 0.028, 0.029, 0.03, 0.031, 0.032, 0.033, 0.034, 0.035, 0.036, 0.037, 0.038, 0.039, 0.04,
    0.041, 0.042, 0.043, 0.044, 0.045, 0.046, 0.047, 0.048, 0.049, 0.05, 0.051, 0.052, 0.053,
    0.054, 0.055, 0.056, 0.057, 0.058, 0.059, 0.06, 0.061, 0.062, 0.063, 0.064, 0.065, 0.066,
    0.067, 0.068, 0.069, 0.07, 0.071, 0.072, 0.073, 0.074, 0.075, 0.076, 0.077, 0.078, 0.079, 0.08,
    0.081, 0.082, 0.083, 0.084, 0.085, 0.086, 0.087, 0.088, 0.089, 0.09, 0.091, 0.092, 0.093,
    0.094, 0.095, 0.096, 0.097, 0.098, 0.099, 0.1, 0.101, 0.102, 0.103, 0.104, 0.105, 0.106, 0.107,
    0.108, 0.109, 0.11, 0.111, 0.112, 0.113, 0.114, 0.115, 0.116, 0.117, 0.118, 0.119, 0.12, 0.121,
    0.122, 0.123, 0.124, 0.125, 0.126, 0.127, 0.128, 0.129, 0.13, 0.131, 0.132, 0.133, 0.134,
    0.135, 0.136, 0.137, 0.138, 0.139, 0.14, 0.141, 0.142, 0.143, 0.144, 0.145, 0.146, 0.147,
    0.148, 0.149, 0.15, 0.151, 0.152, 0.153, 0.154, 0.155, 0.156, 0.157, 0.158, 0.159, 0.16, 0.161,
    0.162, 0.163, 0.164, 0.165, 0.166, 0.167, 0.168, 0.169, 0.17, 0.171, 0.172, 0.173, 0.174,
    0.175, 0.176, 0.177, 0.178, 0.179, 0.18, 0.181, 0.182, 0.183, 0.184, 0.185, 0.186, 0.187,
    0.188, 0.189, 0.19, 0.191, 0.192, 0.193, 0.194, 0.195, 0.196, 0.197, 0.198, 0.199, 0.2, 0.201,
    0.202, 0.203, 0.204, 0.205, 0.206, 0.207, 0.208, 0.209, 0.21, 0.211, 0.212, 0.213, 0.214,
    0.215, 0.216, 0.217, 0.218, 0.219, 0.22, 0.221, 0.222, 0.223, 0.224, 0.225, 0.226, 0.227,
    0.228, 0.229, 0.23, 0.231, 0.232, 0.233, 0.234, 0.235, 0.236, 0.237, 0.238, 0.239, 0.24, 0.241,
    0.242, 0.243, 0.244, 0.245, 0.246, 0.247, 0.248, 0.249, 0.25,
];

// Range: 0.5m
pub(crate) const X0_50M: RangeTable = [
    0.002, 0.004, 0.006, 0.008, 0.01, 0.012, 0.014, 0.016, 0.018, 0.02, 0.022, 0.024, 0.026, 0.028,
    0.03, 0.032, 0.034, 0.036, 0.038, 0.04, 0.042, 0.044, 0.046, 0.048, 0.05, 0.052, 0.054, 0.056,
    0.058, 0.06, 0.062, 0.064, 0.066, 0.068, 0.07, 0.072, 0.074, 0.076, 0.078, 0.08, 0.082, 0.084,
    0.086, 0.088, 0.09, 0.092, 0.094, 0.096, 0.098, 0.1, 0.102, 0.104, 0.106, 0.108, 0.11, 0.112,
    0.114, 0.116, 0.118, 0.12, 0.122, 0.124, 0.126, 0.128, 0.13, 0.132, 0.134, 0.136, 0.138, 0.14,
    0.142, 0.144, 0.146, 0.148, 0.15, 0.152, 0.154, 0.156, 0.158, 0.16, 0.162, 0.164, 0.166, 0.168,
    0.17, 0.172, 0.174, 0.176, 0.178, 0.18, 0.182, 0.184, 0.186, 0.188, 0.19, 0.192, 0.194, 0.196,
    0.198, 0.2, 0.202, 0.204, 0.206, 0.208, 0.21, 0.212, 0.214, 0.216, 0.218, 0.22, 0.222, 0.224,
    0.226, 0.228, 0.23, 0.232, 0.234, 0.236, 0.238, 0.24, 0.242, 0.244, 0.246, 0.248, 0.25, 0.252,
    0.254, 0.256, 0.258, 0.26, 0.262, 0.264, 0.266, 0.268, 0.27, 0.272, 0.274, 0.276, 0.278, 0.28,
    0.282, 0.284, 0.286, 0.288, 0.29, 0.292, 0.294, 0.296, 0.298, 0.3, 0.302, 0.304, 0.306, 0.308,
    0.31, 0.312, 0.314, 0.316, 0.318, 0.32, 0.322, 0.324, 0.326, 0.328, 0.33, 0.332, 0.334, 0.336,
    0.338, 0.34, 0.342, 0.344, 0.346, 0.348, 0.35, 0.352, 0.354, 0.356, 0.358, 0.36, 0.362, 0.364,
    0.366, 0.368, 0.37, 0.372, 0.374, 0.376, 0.378, 0.38, 0.382, 0.384, 0.386, 0.388, 0.39, 0.392,
    0.394, 0.396, 0.398, 0.4, 0.402, 0.404, 0.406, 0.408, 0.41, 0.412, 0.414, 0.416, 0.418, 0.42,
    0.422, 0.424, 0.426, 0.428, 0.43, 0.432, 0.434, 0.436, 0.438, 0.44, 0.442, 0.444, 0.446, 0.448,
    0.45, 0.452, 0.454, 0.456, 0.458, 0.46, 0.462, 0.464, 0.466, 0.468, 0.47, 0.472, 0.474, 0.476,
    0.478, 0.48, 0.482, 0.484, 0.486, 0.488, 0.49, 0.492, 0.494, 0.496, 0.498, 0.5,
];

// Range: 0.75m
pub(crate) const X0_75M: RangeTable = [
    0.003, 0.006, 0.009, 0.012, 0.015, 0.018, 0.021, 0.024, 0.027, 0.03, 0.033, 0.036, 0.039,
    0.042, 0.045, 0.048, 0.051, 0.054, 0.057, 0.06, 0.063, 0.066, 0.069, 0.072, 0.075, 0.078,
    0.081, 0.084, 0.087, 0.09, 0.093, 0.096, 0.099, 0.102, 0.105, 0.108, 0.111, 0.114, 0.117, 0.12,
    0.123, 0.126, 0.129, 0.132, 0.135, 0.138, 0.141, 0.144, 0.147, 0.15, 0.153, 0.156, 0.159,
    0.162, 0.165, 0.168, 0.171, 0.174, 0.177, 0.18, 0.183, 0.186, 0.189, 0.192, 0.195, 0.198,
    0.201, 0.204, 0.207, 0.21, 0.213, 0.216, 0.219, 0.222, 0.225, 0.228, 0.231, 0.234, 0.237, 0.24,
    0.243, 0.246, 0.249, 0.252, 0.255, 0.258, 0.261, 0.264, 0.267, 0.27, 0.273, 0.276, 0.279,
    0.282, 0.285, 0.288, 0.291, 0.294, 0.297, 0.3, 0.303, 0.306, 0.309, 0.312, 0.315, 0.318, 0.321,
    0.324, 0.327, 0.33, 0.333, 0.336, 0.339, 0.342, 0.345, 0.348, 0.351, 0.354, 0.357, 0.36, 0.363,
    0.366, 0.369, 0.372, 0.375, 0.378, 0.381, 0.384, 0.387, 0.39, 0.393, 0.396, 0.399, 0.402,
    0.405, 0.408, 0.411, 0.414, 0.417, 0.42, 0.423, 0.426, 0.429, 0.432, 0.435, 0.438, 0.441,
    0.444, 0.447, 0.45, 0.453, 0.456, 0.459, 0.462, 0.465, 0.468, 0.471, 0.474, 0.477, 0.48, 0.483,
    0.486, 0.489, 0.492, 0.495, 0.498, 0.501, 0.504, 0.507, 0.51, 0.513, 0.516, 0.519, 0.522,
    0.525, 0.528, 0.531, 0.534, 0.537, 0.54, 0.543, 0.546, 0.549, 0.552, 0.555, 0.558, 0.561,
    0.564, 0.567, 0.57, 0.573, 0.576, 0.579, 0.582, 0.585, 0.588, 0.591, 0.594, 0.597, 0.6, 0.603,
    0.606, 0.609, 0.612, 0.615, 0.618, 0.621, 0.624, 0.627, 0.63, 0.633, 0.636, 0.639, 0.642,
    0.645, 0.648, 0.651, 0.654, 0.657, 0.66, 0.663, 0.666, 0.669, 0.672, 0.675, 0.678, 0.681,
    0.684, 0.687, 0.69, 0.693, 0.696, 0.699, 0.702, 0.705, 0.708, 0.711, 0.714, 0.717, 0.72, 0.723,
    0.726, 0.729, 0.732, 0.735, 0.738, 0.741, 0.744, 0.747, 0.75,
];

// Range: 1m
pub(crate) const X1M: RangeTable = [
    0.004, 0.008, 0.012, 0.016, 0.02, 0.024, 0.028, 0.032, 0.036, 0.04, 0.044, 0.048, 0.052, 0.056,
    0.06, 0.064, 0.068, 0.072, 0.076, 0.08, 0.084, 0.088, 0.092, 0.096, 0.1, 0.104, 0.108, 0.112,
    0.116, 0.12, 0.124, 0.128, 0.132, 0.136, 0.14, 0.144, 0.148, 0.152, 0.156, 0.16, 0.164, 0.168,
    0.172, 0.176, 0.18, 0.184, 0.188, 0.192, 0.196, 0.2, 0.204, 0.208, 0.212, 0.216, 0.22, 0.224,
    0.228, 0.232, 0.236, 0.24, 0.244, 0.248, 0.252, 0.256, 0.26, 0.264, 0.268, 0.272, 0.276, 0.28,
    0.284, 0.288, 0.292, 0.296, 0.3, 0.304, 0.308, 0.312, 0.316, 0.32, 0.324, 0.328, 0.332, 0.336,
    0.34, 0.344, 0.348, 0.352, 0.356, 0.36, 0.364, 0.368, 0.372, 0.376, 0.38, 0.384, 0.388, 0.392,
    0.396, 0.4, 0.404, 0.408, 0.412, 0.416, 0.42, 0.424, 0.428, 0.432, 0.436, 0.44, 0.444, 0.448,
    0.452, 0.456, 0.46, 0.464, 0.468, 0.472, 0.476, 0.48, 0.484, 0.488, 0.492, 0.496, 0.5, 0.504,
    0.508, 0.512, 0.516, 0.52, 0.524, 0.528, 0.532, 0.536, 0.54, 0.544, 0.548, 0.552, 0.556, 0.56,
    0.564, 0.568, 0.572, 0.576, 0.58, 0.584, 0.588, 0.592, 0.596, 0.6, 0.604, 0.608, 0.612, 0.616,
    0.62, 0.624, 0.628, 0.632, 0.636, 0.64, 0.644, 0.648, 0.652, 0.656, 0.66, 0.664, 0.668, 0.672,
    0.676, 0.68, 0.684, 0.688, 0.692, 0.696, 0.7, 0.704, 0.708, 0.712, 0.716, 0.72, 0.724, 0.728,
    0.732, 0.736, 0.74, 0.744, 0.748, 0.752, 0.756, 0.76, 0.764, 0.768, 0.772, 0.776, 0.78, 0.784,
    0.788, 0.792, 0.796, 0.8, 0.804, 0.808, 0.812, 0.816, 0.82, 0.824, 0.828, 0.832, 0.836, 0.84,
    0.844, 0.848, 0.852, 0.856, 0.86, 0.864, 0.868, 0.872, 0.876, 0.88, 0.884, 0.888, 0.892, 0.896,
    0.9, 0.904, 0.908, 0.912, 0.916, 0.92, 0.924, 0.928, 0.932, 0.936, 0.94, 0.944, 0.948, 0.952,
    0.956, 0.96, 0.964, 0.968, 0.972, 0.976, 0.98, 0.984, 0.988, 0.992, 0.996, 1.0,
];

// Range: 2m
pub(crate) const X2M: RangeTable = [
    0.008, 0.016, 0.024, 0.032, 0.04, 0.048, 0.056, 0.064, 0.072, 0.08, 0.088, 0.096, 0.104, 0.112,
    0.12, 0.128, 0.136, 0.144, 0.152, 0.16, 0.168, 0.176, 0.184, 0.192, 0.2, 0.208, 0.216, 0.224,
    0.232, 0.24, 0.248, 0.256, 0.264, 0.272, 0.28, 0.288, 0.296, 0.304, 0.312, 0.32, 0.328, 0.336,
    0.344, 0.352, 0.36, 0.368, 0.376, 0.384, 0.392, 0.4, 0.408, 0.416, 0.424, 0.432, 0.44, 0.448,
    0.456, 0.464, 0.472, 0.48, 0.488, 0.496, 0.504, 0.512, 0.52, 0.528, 0.536, 0.544, 0.552, 0.56,
    0.568, 0.576, 0.584, 0.592, 0.6, 0.608, 0.616, 0.624, 0.632, 0.64, 0.648, 0.656, 0.664, 0.672,
    0.68, 0.688, 0.696, 0.704, 0.712, 0.72, 0.728, 0.736, 0.744, 0.752, 0.76, 0.768, 0.776, 0.784,
    0.792, 0.8, 0.808, 0.816, 0.824, 0.832, 0.84, 0.848, 0.856, 0.864, 0.872, 0.88, 0.888, 0.896,
    0.904, 0.912, 0.92, 0.928, 0.936, 0.944, 0.952, 0.96, 0.968, 0.976, 0.984, 0.992, 1.0, 1.008,
    1.016, 1.024, 1.032, 1.04, 1.048, 1.056, 1.064, 1.072, 1.08, 1.088, 1.096, 1.104, 1.112, 1.12,
    1.128, 1.136, 1.144, 1.152, 1.16, 1.168, 1.176, 1.184, 1.192, 1.2, 1.208, 1.216, 1.224, 1.232,
    1.24, 1.248, 1.256, 1.264, 1.272, 1.28, 1.288, 1.296, 1.304, 1.312, 1.32, 1.328, 1.336, 1.344,
    1.352, 1.36, 1.368, 1.376, 1.384, 1.392, 1.4, 1.408, 1.416, 1.424, 1.432, 1.44, 1.448, 1.456,
    1.464, 1.472, 1.48, 1.488, 1.496, 1.504, 1.512, 1.52, 1.528, 1.536, 1.544, 1.552, 1.56, 1.568,
    1.576, 1.584, 1.592, 1.6, 1.608, 1.616, 1.624, 1.632, 1.64, 1.648, 1.656, 1.664, 1.672, 1.68,
    1.688, 1.696, 1.704, 1.712, 1.72, 1.728, 1.736, 1.744, 1.752, 1.76, 1.768, 1.776, 1.784, 1.792,
    1.8, 1.808, 1.816, 1.824, 1.832, 1.84, 1.848, 1.856, 1.864, 1.872, 1.88, 1.888, 1.896, 1.904,
    1.912, 1.92, 1.928, 1.936, 1.944, 1.952, 1.96, 1.968, 1.976, 1.984, 1.992, 2.0,
];

// Range: 3m
pub(crate) const X3M: RangeTable = [
    0.012, 0.024, 0.036, 0.048, 0.06, 0.072, 0.084, 0.096, 0.108, 0.12, 0.132, 0.144, 0.156, 0.168,
    0.18, 0.192, 0.204, 0.216, 0.228, 0.24, 0.252, 0.264, 0.276, 0.288, 0.3, 0.312, 0.324, 0.336,
    0.348, 0.36, 0.372, 0.384, 0.396, 0.408, 0.42, 0.432, 0.444, 0.456, 0.468, 0.48, 0.492, 0.504,
    0.516, 0.528, 0.54, 0.552, 0.564, 0.576, 0.588, 0.6, 0.612, 0.624, 0.636, 0.648, 0.66, 0.672,
    0.684, 0.696, 0.708, 0.72, 0.732, 0.744, 0.756, 0.768, 0.78, 0.792, 0.804, 0.816, 0.828, 0.84,
    0.852, 0.864, 0.876, 0.888, 0.9, 0.912, 0.924, 0.936, 0.948, 0.96, 0.972, 0.984, 0.996, 1.008,
    1.02, 1.032, 1.044, 1.056, 1.068, 1.08, 1.092, 1.104, 1.116, 1.128, 1.14, 1.152, 1.164, 1.176,
    1.188, 1.2, 1.212, 1.224, 1.236, 1.248, 1.26, 1.272, 1.284, 1.296, 1.308, 1.32, 1.332, 1.344,
    1.356, 1.368, 1.38, 1.392, 1.404, 1.416, 1.428, 1.44, 1.452, 1.464, 1.476, 1.488, 1.5, 1.512,
    1.524, 1.536, 1.548, 1.56, 1.572, 1.584, 1.596, 1.608, 1.62, 1.632, 1.644, 1.656, 1.668, 1.68,
    1.692, 1.704, 1.716, 1.728, 1.74, 1.752, 1.764, 1.776, 1.788, 1.8, 1.812, 1.824, 1.836, 1.848,
    1.86, 1.872, 1.884, 1.896, 1.908, 1.92, 1.932, 1.944, 1.956, 1.968, 1.98, 1.992, 2.004, 2.016,
    2.028, 2.04, 2.052, 2.064, 2.076, 2.088, 2.1, 2.112, 2.124, 2.136, 2.148, 2.16, 2.172, 2.184,
    2.196, 2.208, 2.22, 2.232, 2.244, 2.256, 2.268, 2.28, 2.292, 2.304, 2.316, 2.328, 2.34, 2.352,
    2.364, 2.376, 2.388, 2.4, 2.412, 2.424, 2.436, 2.448, 2.46, 2.472, 2.484, 2.496, 2.508, 2.52,
    2.532, 2.544, 2.556, 2.568, 2.58, 2.592, 2.604, 2.616, 2.628, 2.64, 2.652, 2.664, 2.676, 2.688,
    2.7, 2.712, 2.724, 2.736, 2.748, 2.76, 2.772, 2.784, 2.796, 2.808, 2.82, 2.832, 2.844, 2.856,
    2.868, 2.88, 2.892, 2.904, 2.916, 2.928, 2.94, 2.952, 2.964, 2.976, 2.988, 3.0,
];

// Range: 4m
pub(crate) const X4M: RangeTable = [
    0.016, 0.032, 0.048, 0.064, 0.08, 0.096, 0.112, 0.128, 0.144, 0.16, 0.176, 0.192, 0.208, 0.224,
    0.24, 0.256, 0.272, 0.288, 0.304, 0.32, 0.336, 0.352, 0.368, 0.384, 0.4, 0.416, 0.432, 0.448,
    0.464, 0.48, 0.496, 0.512, 0.528, 0.544, 0.56, 0.576, 0.592, 0.608, 0.624, 0.64, 0.656, 0.672,
    0.688, 0.704, 0.72, 0.736, 0.752, 0.768, 0.784, 0.8, 0.816, 0.832, 0.848, 0.864, 0.88, 0.896,
    0.912, 0.928, 0.944, 0.96, 0.976, 0.992, 1.008, 1.024, 1.04, 1.056, 1.072, 1.088, 1.104, 1.12,
    1.136, 1.152, 1.168, 1.184, 1.2, 1.216, 1.232, 1.248, 1.264, 1.28, 1.296, 1.312, 1.328, 1.344,
    1.36, 1.376, 1.392, 1.408, 1.424, 1.44, 1.456, 1.472, 1.488, 1.504, 1.52, 1.536, 1.552, 1.568,
    1.584, 1.6, 1.616, 1.632, 1.648, 1.664, 1.68, 1.696, 1.712, 1.728, 1.744, 1.76, 1.776, 1.792,
    1.808, 1.824, 1.84, 1.856, 1.872, 1.888, 1.904, 1.92, 1.936, 1.952, 1.968, 1.984, 2.0, 2.016,
    2.032, 2.048, 2.064, 2.08, 2.096, 2.112, 2.128, 2.144, 2.16, 2.176, 2.192, 2.208, 2.224, 2.24,
    2.256, 2.272, 2.288, 2.304, 2.32, 2.336, 2.352, 2.368, 2.384, 2.4, 2.416, 2.432, 2.448, 2.464,
    2.48, 2.496, 2.512, 2.528, 2.544, 2.56, 2.576, 2.592, 2.608, 2.624, 2.64, 2.656, 2.672, 2.688,
    2.704, 2.72, 2.736, 2.752, 2.768, 2.784, 2.8, 2.816, 2.832, 2.848, 2.864, 2.88, 2.896, 2.912,
    2.928, 2.944, 2.96, 2.976, 2.992, 3.008, 3.024, 3.04, 3.056, 3.072, 3.088, 3.104, 3.12, 3.136,
    3.152, 3.168, 3.184, 3.2, 3.216, 3.232, 3.248, 3.264, 3.28, 3.296, 3.312, 3.328, 3.344, 3.36,
    3.376, 3.392, 3.408, 3.424, 3.44, 3.456, 3.472, 3.488, 3.504, 3.52, 3.536, 3.552, 3.568, 3.584,
    3.6, 3.616, 3.632, 3.648, 3.664, 3.68, 3.696, 3.712, 3.728, 3.744, 3.76, 3.776, 3.792, 3.808,
    3.824, 3.84, 3.856, 3.872, 3.888, 3.904, 3.92, 3.936, 3.952, 3.968, 3.984, 4.0,
];

// Range: 5m
pub(crate) const X5M: RangeTable = [
    0.02, 0.04, 0.06, 0.08, 0.1, 0.12, 0.14, 0.16, 0.18, 0.2, 0.22, 0.24, 0.26, 0.28, 0.3, 0.32,
    0.34, 0.36, 0.38, 0.4, 0.42, 0.44, 0.46, 0.48, 0.5, 0.52, 0.54, 0.56, 0.58, 0.6, 0.62, 0.64,
    0.66, 0.68, 0.7, 0.72, 0.74, 0.76, 0.78, 0.8, 0.82, 0.84, 0.86, 0.88, 0.9, 0.92, 0.94, 0.96,
    0.98, 1.0, 1.02, 1.04, 1.06, 1.08, 1.1, 1.12, 1.14, 1.16, 1.18, 1.2, 1.22, 1.24, 1.26, 1.28,
    1.3, 1.32, 1.34, 1.36, 1.38, 1.4, 1.42, 1.44, 1.46, 1.48, 1.5, 1.52, 1.54, 1.56, 1.58, 1.6,
    1.62, 1.64, 1.66, 1.68, 1.7, 1.72, 1.74, 1.76, 1.78, 1.8, 1.82, 1.84, 1.86, 1.88, 1.9, 1.92,
    1.94, 1.96, 1.98, 2.0, 2.02, 2.04, 2.06, 2.08, 2.1, 2.12, 2.14, 2.16, 2.18, 2.2, 2.22, 2.24,
    2.26, 2.28, 2.3, 2.32, 2.34, 2.36, 2.38, 2.4, 2.42, 2.44, 2.46, 2.48, 2.5, 2.52, 2.54, 2.56,
    2.58, 2.6, 2.62, 2.64, 2.66, 2.68, 2.7, 2.72, 2.74, 2.76, 2.78, 2.8, 2.82, 2.84, 2.86, 2.88,
    2.9, 2.92, 2.94, 2.96, 2.98, 3.0, 3.02, 3.04, 3.06, 3.08, 3.1, 3.12, 3.14, 3.16, 3.18, 3.2,
    3.22, 3.24, 3.26, 3.28, 3.3, 3.32, 3.34, 3.36, 3.38, 3.4, 3.42, 3.44, 3.46, 3.48, 3.5, 3.52,
    3.54, 3.56, 3.58, 3.6, 3.62, 3.64, 3.66, 3.68, 3.7, 3.72, 3.74, 3.76, 3.78, 3.8, 3.82, 3.84,
    3.86, 3.88, 3.9, 3.92, 3.94, 3.96, 3.98, 4.0, 4.02, 4.04, 4.06, 4.08, 4.1, 4.12, 4.14, 4.16,
    4.18, 4.2, 4.22, 4.24, 4.26, 4.28, 4.3, 4.32, 4.34, 4.36, 4.38, 4.4, 4.42, 4.44, 4.46, 4.48,
    4.5, 4.52, 4.54, 4.56, 4.58, 4.6, 4.62, 4.64, 4.66, 4.68, 4.7, 4.72, 4.74, 4.76, 4.78, 4.8,
    4.82, 4.84, 4.86, 4.88, 4.9, 4.92, 4.94, 4.96, 4.98, 5.0,
];

// Range: 6m
pub(crate) const X6M: RangeTable = [
    0.024, 0.048, 0.072, 0.096, 0.12, 0.144, 0.168, 0.192, 0.216, 0.24, 0.264, 0.288, 0.312, 0.336,
    0.36, 0.384, 0.408, 0.432, 0.456, 0.48, 0.504, 0.528, 0.552, 0.576, 0.6, 0.624, 0.648, 0.672,
    0.696, 0.72, 0.744, 0.768, 0.792, 0.816, 0.84, 0.864, 0.888, 0.912, 0.936, 0.96, 0.984, 1.008,
    1.032, 1.056, 1.08, 1.104, 1.128, 1.152, 1.176, 1.2, 1.224, 1.248, 1.272, 1.296, 1.32, 1.344,
    1.368, 1.392, 1.416, 1.44, 1.464, 1.488, 1.512, 1.536, 1.56, 1.584, 1.608, 1.632, 1.656, 1.68,
    1.704, 1.728, 1.752, 1.776, 1.8, 1.824, 1.848, 1.872, 1.896, 1.92, 1.944, 1.968, 1.992, 2.016,
    2.04, 2.064, 2.088, 2.112, 2.136, 2.16, 2.184, 2.208, 2.232, 2.256, 2.28, 2.304, 2.328, 2.352,
    2.376, 2.4, 2.424, 2.448, 2.472, 2.496, 2.52, 2.544, 2.568, 2.592, 2.616, 2.64, 2.664, 2.688,
    2.712, 2.736, 2.76, 2.784, 2.808, 2.832, 2.856, 2.88, 2.904, 2.928, 2.952, 2.976, 3.0, 3.024,
    3.048, 3.072, 3.096, 3.12, 3.144, 3.168, 3.192, 3.216, 3.24, 3.264, 3.288, 3.312, 3.336, 3.36,
    3.384, 3.408, 3.432, 3.456, 3.48, 3.504, 3.528, 3.552, 3.576, 3.6, 3.624, 3.648, 3.672, 3.696,
    3.72, 3.744, 3.768, 3.792, 3.816, 3.84, 3.864, 3.888, 3.912, 3.936, 3.96, 3.984, 4.008, 4.032,
    4.056, 4.08, 4.104, 4.128, 4.152, 4.176, 4.2, 4.224, 4.248, 4.272, 4.296, 4.32, 4.344, 4.368,
    4.392, 4.416, 4.44, 4.464, 4.488, 4.512, 4.536, 4.56, 4.584, 4.608, 4.632, 4.656, 4.68, 4.704,
    4.728, 4.752, 4.776, 4.8, 4.824, 4.848, 4.872, 4.896, 4.92, 4.944, 4.968, 4.992, 5.016, 5.04,
    5.064, 5.088, 5.112, 5.136, 5.16, 5.184, 5.208, 5.232, 5.256, 5.28, 5.304, 5.328, 5.352, 5.376,
    5.4, 5.424, 5.448, 5.472, 5.496, 5.52, 5.544, 5.568, 5.592, 5.616, 5.64, 5.664, 5.688, 5.712,
    5.736, 5.76, 5.784, 5.808, 5.832, 5.856, 5.88, 5.904, 5.928, 5.952, 5.976, 6.0,
];
