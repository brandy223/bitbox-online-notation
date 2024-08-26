import React, {useState} from "react";
import {capitalizeFirstLetter, hideModal} from "@/app/utils";
import {Student} from "@/app/api/models/student";

interface NewProjectModalProps {
    students: Student[];
    setStudents: React.Dispatch<React.SetStateAction<Student[]>>;
    student: Student | null
}

const DeleteStudentModal: React.FC<NewProjectModalProps> = ({students, setStudents, student}) => {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            if (student === null) {
                setError("An error occurred. Please try again later.");
                return;
            }

            const response = await fetch(`http://localhost:8080/api/students/${student.id}`, {
                method: "DELETE",
                headers: {"Content-Type": "application/json"},
                credentials: "include",
            });

            if (response.status === 200) {
                const newStudents = students.filter(student_ => student_.id !== student.id);
                setStudents(newStudents);
                hideModal("delete_student_modal");
            } else {
                setError("An error occurred. Please try again later.");
            }
        } catch (err) {
            setError("An error occurred. Please try again later.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <dialog id="delete_student_modal" className="modal">
            <div className="modal-box">
                {student === null ? (
                    <h3 className="font-bold text-lg">No student selected</h3>
                ) : (
                    <h3 className="font-bold text-lg">Confirm removal of
                        student {`"`}{student.name.toUpperCase()} {capitalizeFirstLetter(student.surname)}{`"`}</h3>
                )}
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p style={{color: "red"}}>{error}</p>
                ): null}
                <form onSubmit={handleSubmit} className={"flex flex-col"}>
                    <div className="my-4 flex justify-end w-full">
                        {student === null ? null : (
                            <button type={"submit"}
                                    className={"flex- rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                                Confirm
                            </button>
                        )}
                        <button onClick={() => hideModal("delete_student_modal")}
                                className={"flex- rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                            Cancel
                        </button>
                    </div>
                </form>
            </div>
            <form method="dialog" className="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    )
}

export default DeleteStudentModal;