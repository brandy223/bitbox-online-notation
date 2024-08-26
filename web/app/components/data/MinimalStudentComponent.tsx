import React from "react";
import {StudentGroup} from "@/app/api/models/student-group";
import {capitalizeFirstLetter} from "@/app/utils";
import Link from "next/link";

const MinimalStudentComponent: React.FC<{
    group_id: string,
    student_id: string
    student_details: StudentGroup;
    withMark: boolean;
}> = ({ group_id, student_id, student_details, withMark }) => {
    return (
        <Link href={`/group/${group_id}/student/${student_id}`} key={student_details.student.id}>
            <div className="flex-row">
                <p>{student_details.student.name.toUpperCase()}</p>
                <p>{capitalizeFirstLetter(student_details.student.surname)}</p>
                {withMark ? <p>{student_details.mark}</p> : null}
            </div>
        </Link>
    );
};

export default MinimalStudentComponent;
