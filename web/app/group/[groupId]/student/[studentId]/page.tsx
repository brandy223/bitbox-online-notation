'use client'

import React, {useEffect, useState} from "react";
import {useParams} from "next/navigation";
import {StudentGroupMarkDetails} from "@/app/api/models/student-group-mark-details";
import {capitalizeFirstLetter} from "@/app/utils";
import NavBar from "@/app/components/NavBar";

async function getStudentMarks(group_id: string, studentId: string): Promise<StudentGroupMarkDetails> {
    const api_url = process.env.NEXT_PUBLIC_API_URL;
    const response = await fetch(`${api_url}/groups/${group_id}/student/${studentId}`, {
        headers: {"Content-Type": "application/json"},
        credentials: "include",
    });
    const data = await response.json();
    return data as StudentGroupMarkDetails;
}

const StudentDetailsComponent: React.FC<{name: string, surname: string}> = ({name, surname}) => {

    return (
        <div className="flex-row">
            <p>{name.toUpperCase()}</p>
            <p>{capitalizeFirstLetter(surname)}</p>
        </div>
    );
}

interface MarkDetailsProps {
    mark: null | number | undefined
    name: string;
    surname: string;
    comment: string | null | undefined;
}

const MarkDetailsComponent: React.FC<MarkDetailsProps> = ({ mark, name, surname, comment }) => {
    return (
        <div className="border rounded-lg shadow p-4 mb-4">
            <div className="grid grid-cols-3 gap-4">
                <div className="col-span-1">
                    <h3 className="font-semibold">Student Name</h3>
                    <p>{name.toUpperCase()} {capitalizeFirstLetter(surname)}</p>
                </div>
                <div className="col-span-1">
                    <h3 className="font-semibold">Mark</h3>
                    <p>{mark !== null && mark !== undefined ? mark : "N/A"}</p>
                </div>
                <div className="col-span-1">
                    <h3 className="font-semibold">Comment</h3>
                    <p>{comment ? comment : "No comment provided"}</p>
                </div>
            </div>
        </div>
    );
}

const StudentMarkDetailsPage: React.FC = () => {
    const {groupId, studentId } = useParams<{ groupId: string, studentId: string }>();
    const [studentGroupMarkDetails, setStudentGroupMarkDetails] = useState<StudentGroupMarkDetails>({
        student: {
            email: "",
            id: "",
            name: "",
            surname: ""
        },
        marks: [],
    });
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchStudentMarks = async () => {
            setLoading(true);
            setError(null);
            try {
                const studentMarks = await getStudentMarks(groupId, studentId);
                setStudentGroupMarkDetails(studentMarks);
            } catch (err) {
                setError("An error occurred. Please try again later.");
            } finally {
                setLoading(false);
            }
        }

        fetchStudentMarks();
    }, [groupId, studentId]);

    return (
        <div>
            <NavBar />
            {loading ? (
                <p>Loading...</p>
            ) : error ? (
                <p>{error}</p>
            ) : (
                <>
                    <StudentDetailsComponent
                        name={studentGroupMarkDetails.student.name}
                        surname={studentGroupMarkDetails.student.surname}
                        key={groupId}
                    />
                    {studentGroupMarkDetails.marks.map((mark) => (
                        <MarkDetailsComponent
                            key={studentGroupMarkDetails.student.id} // Added a unique key prop here
                            mark={mark.mark}
                            name={mark.grader.name}
                            surname={mark.grader.surname}
                            comment={mark.comment}
                        />
                    ))}
                </>
            )}
        </div>
    );

}

export default StudentMarkDetailsPage;