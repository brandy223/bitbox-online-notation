import React, {useState} from "react";
import {hideModal} from "@/app/utils";
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
            <div className="modal-box rounded-lg shadow-lg bg-gray-200">
                <h3 className="font-bold text-lg text-gray-800">Confirm Removal</h3>
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p className="text-red-600">{error}</p>
                ) : null}
                <form onSubmit={handleSubmit} className="space-y-4">
                    {student ? (
                        <p className="text-gray-600">Are you sure you want to remove {student.name} {student.surname}?</p>
                    ) : (
                        <p>No student selected.</p>
                    )}
                    <div className="flex justify-end space-x-4">
                        <button type="button" onClick={() => hideModal("delete_student_modal")}
                                className="bg-gray-300 text-gray-700 rounded-lg px-4 py-2 hover:bg-gray-400 transition">
                            Cancel
                        </button>
                        <button type="submit"
                                className="bg-red-500 text-white rounded-lg px-4 py-2 hover:bg-red-600 transition">
                            Confirm
                        </button>
                    </div>
                </form>
            </div>
        </dialog>

    )
}

export default DeleteStudentModal;