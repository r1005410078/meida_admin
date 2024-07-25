-- Your SQL goes here
CREATE TABLE houses (
    house_id INT AUTO_INCREMENT PRIMARY KEY, -- 房屋编号，自动递增
    neighborhood_name VARCHAR(255) NOT NULL, -- 小区编号
    house_address VARCHAR(255) NOT NULL, -- 房屋地址
    house_type VARCHAR(50) NOT NULL, -- 房屋类型
    area DECIMAL(10, 2) NOT NULL, -- 房屋面积，保留两位小数
    bedrooms INT NOT NULL, -- 卧室数量
    living_rooms INT NOT NULL, -- 客厅数量
    bathrooms INT NOT NULL, -- 卫生间数量
    orientation VARCHAR(20), -- 房屋朝向
    decoration_status VARCHAR(50), -- 房屋装修情况
    status VARCHAR(50), -- 房屋状态
    house_description TEXT, -- 房屋描述
    house_image VARCHAR(255), -- 房屋图片
    owner_name VARCHAR(100) NOT NULL, -- 户主姓名
    owner_phone VARCHAR(20) NOT NULL, -- 户主联系方式
    deleted_at DATETIME, -- 删除时间
    created_by VARCHAR(255) NOT NULL, -- 创建人
    updated_by VARCHAR(255) NOT NULL, -- 更新人
    deleted_by VARCHAR(255),          -- 删除人
    created_at DATETIME NOT NULL, -- 创建时间
    updated_at DATETIME NOT NULL -- 更新时间
);