ALTER TABLE `project_labels` DROP FOREIGN KEY `fk_project_label_user_id`;
ALTER TABLE `project_labels` DROP FOREIGN KEY `fk_project_label_label_id`;
ALTER TABLE `project_labels` DROP FOREIGN KEY `fk_project_label_project_id`;
DROP TABLE `project_labels`;