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
            <div className="flex justify-between items-center p-3 bg-gray-100 rounded-lg shadow hover:bg-gray-200 transition">
                <div>
                    <p className="text-lg font-medium text-gray-800">{student_details.student.name.toUpperCase()}</p>
                    <p className="text-sm text-gray-600">{capitalizeFirstLetter(student_details.student.surname)}</p>
                </div>
                {withMark && (
                    <p className="text-lg font-bold text-gray-700">{student_details.mark}</p>
                )}
            </div>
        </Link>
    );
};

export default MinimalStudentComponent;
