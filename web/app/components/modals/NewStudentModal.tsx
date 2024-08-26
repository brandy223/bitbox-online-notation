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
            <div className="modal-box">
                <h3 className="font-bold text-lg">Add a student</h3>
                <div className="divider"></div>
                {loading ? (
                    <p>Loading...</p>
                ) : error ? (
                    <p style={{color: "red"}}>{error}</p>
                ): null}
                <form onSubmit={handleSubmit} className={"flex flex-col"}>
                    <div className="my-5 flex-row ">
                        <input
                            type="text"
                            placeholder="Student name"
                            name="name"
                            id="name"
                            value={formData.name}
                            required
                            maxLength={64}
                            onChange={handleInputChange}
                            className="input input-bordered w-full max-w-xs"
                        />
                        <input
                            type="text"
                            placeholder="Student surname"
                            name="surname"
                            id="surname"
                            value={formData.surname}
                            required
                            maxLength={64}
                            onChange={handleInputChange}
                            className="input input-bordered w-full max-w-xs"
                        />
                    </div>
                    <input
                        type="email"
                        placeholder="Student email"
                        name="email"
                        id="email"
                        value={formData.email}
                        required
                        maxLength={128}
                        onChange={handleInputChange}
                        className="input input-bordered w-full max-w-xs"
                    />
                    <div className="my-4 flex justify-end w-full">
                        <button type={"submit"}
                                className={"flex- rounded-full hover:bg-base-200 px-4 py-2 font-bold w-32"}>
                            Submit
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

export default NewStudentModal;