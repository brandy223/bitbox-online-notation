/* tslint:disable */
/* eslint-disable */
import {Student} from '../models/student';
import {StudentGroupMark} from '../models/student-group-mark';

export interface StudentGroupMarkDetails {
  marks: Array<StudentGroupMark>;
  student: Student;
}
