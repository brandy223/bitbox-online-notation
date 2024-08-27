import React, {useState} from "react";
import {Student} from "@/app/api/models/student";
import {hideModal} from "@/app/utils";

interface NewStudentModalProps {
    students: Student[];
    setStudents: React.Dispatch<React.SetStateAction<Student[]>>;
    promotion_id: string;
}

const NewStudentModal: React.FC<NewStudentModalProps> = ({students, setStudents, promotion_id}) => {
    const [formData, setFormData] = useState({
        name: '',
        surname: '',
        email: '',
    });

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const {name, value} = e.target;
        setFormData((prevData) => ({...prevData, [name]: value}));
    };

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setError(null);
        setLoading(true);
        try {
            const response = await fetch(`http://localhost:8080/api/students/promotion/${promotion_id}`, {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(formData),
                credentials: "include",
            });

            if (response.status === 201) {
                const id: string = await response.json();
                const newStudent: Student = {
                    id,
                    name: formData.name,
                    email: formData.email,
                    surname: formData.surname,
                }
                setStudents([...students, newStudent]);
                hideModal("new_student_modal");
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
        <dialog id="new_student_modal" className="modal">
            <div className="modal-box rounded-lg shadow-lg bg-gray-200">
                <h3 className="font-bold text-xl text-gray-800">Add a Student</h3>
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p className="text-red-600">{error}</p>
                ) : null}
                <form onSubmit={handleSubmit} className="space-y-4">
                    <input
                        type="text"
                        placeholder="Student name"
                        name="name"
                        value={formData.name}
                        onChange={handleInputChange}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg shadow-sm focus:ring focus:ring-blue-200 transition"
                    />
                    <input
                        type="text"
                        placeholder="Student surname"
                        name="surname"
                        value={formData.surname}
                        onChange={handleInputChange}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg shadow-sm focus:ring focus:ring-blue-200 transition"
                    />
                    <input
                        type="email"
                        placeholder="Student email"
                        name="email"
                        value={formData.email}
                        onChange={handleInputChange}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg shadow-sm focus:ring focus:ring-blue-200 transition"
                    />
                    <div className="flex justify-end space-x-4">
                        <button type="button" onClick={() => hideModal("new_student_modal")}
                                className="bg-gray-300 text-gray-700 rounded-lg px-4 py-2 hover:bg-gray-400 transition">
                            Cancel
                        </button>
                        <button type="submit"
                                className="bg-blue-500 text-white rounded-lg px-4 py-2 hover:bg-blue-600 transition">
                            Submit
                        </button>
                    </div>
                </form>
            </div>
        </dialog>

    )
}

export default NewStudentModal;