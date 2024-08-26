/* tslint:disable */
/* eslint-disable */
import {Group} from '../models/group';
import {StudentGroup} from '../models/student-group';

export interface ProjectGroup {
  group: Group;
  students: Array<StudentGroup>;
}
